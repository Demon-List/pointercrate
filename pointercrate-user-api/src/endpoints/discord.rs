use std::net::IpAddr;

use rocket::{
    http::{ContentType, Cookie, CookieJar, SameSite},
    time::Duration,
};

use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse as _};
use pointercrate_core::{
    error::CoreError,
    pool::{audit_connection, PointercratePool},
};
use pointercrate_core_api::{error::Result, response::Response2};
use pointercrate_core_macros::localized;
use pointercrate_demonlist::{
    error::DemonlistError,
    player::{channel::PlayerChannels, claim::PlayerClaim},
};
use pointercrate_user::{
    auth::ApiToken,
    discord::{get_discord_client, DiscordAuthRequestData, DiscordAuthUrl, DiscordConnection},
    error::UserError::{self, HttpClientFailed},
};
use rocket::{serde::json::Json, State};

use crate::{auth::Auth, ratelimits::UserRatelimits};

/// new cookie binding a discord oauth flow to the browser that initiated it
///
/// we hit the callback endpoint as a redirect from discord and the normal access token has
/// `SameSite=Strict` so it doesn't function properly. this cookie is `SameSite=Lax` and
/// must match the oauth `state` parameter, which identifies the member that initiated the flow
const DISCORD_STATE_COOKIE: &str = "discord_oauth_state";

#[localized]
#[rocket::post("/discord/")]
pub async fn link_discord(
    mut auth: Auth<ApiToken>, ratelimits: &State<UserRatelimits>, ip: IpAddr, cookies: &CookieJar<'_>,
) -> Result<Json<DiscordAuthUrl>> {
    ratelimits.discord_link_attempts(ip)?;

    let member_id = auth.user.user().id;

    if !PlayerClaim::by_user(member_id, &mut auth.connection)
        .await?
        .is_some_and(|claim| !claim.verified)
    {
        return Err(UserError::DiscordAuthNoUnverifiedPlayerClaim.into());
    }

    let client = get_discord_client();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("connections".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let state = csrf_token.into_secret();

    DiscordAuthRequestData::create(
        &mut auth.connection,
        DiscordAuthRequestData::build(state.clone(), pkce_verifier.into_secret()),
        member_id,
    )
    .await?;
    auth.commit().await?;

    cookies.add(
        Cookie::build((DISCORD_STATE_COOKIE, state))
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(!cfg!(debug_assertions))
            .path("/api/v1/users/discord/")
            .max_age(Duration::minutes(10)),
    );

    Ok(Json(DiscordAuthUrl { url: auth_url.to_string() }))
}

#[localized]
#[rocket::get("/discord/callback/?<code>&<state>")]
// we get here as a redirect from discord, so the normal access token cookie and the
// X-CSRF-TOKEN header are unavailable. the member is identified via the OAuth `state` parameter and the
// request is bound to the initiating browser via the DISCORD_STATE_COOKIE set in `link_discord`
pub async fn link_discord_callback(
    pool: &State<PointercratePool>, cookies: &CookieJar<'_>, code: String, state: String,
) -> Result<Response2<(ContentType, ())>> {
    let state_cookie = cookies.get(DISCORD_STATE_COOKIE).map(|cookie| cookie.value().to_string());
    cookies.remove(Cookie::build(DISCORD_STATE_COOKIE).path("/api/v1/users/discord/"));

    if state_cookie.as_deref() != Some(state.as_str()) {
        return Err(UserError::DiscordAuthInitiatorMismatch.into());
    }

    let mut pool_connection = pool.connection().await?;
    let client = get_discord_client();
    let code = AuthorizationCode::new(code);

    // This should not get rolled back on error
    let (request_data, member_id) = DiscordAuthRequestData::take(&mut pool_connection, &state).await?;

    let mut connection = pool.transaction().await?;
    audit_connection(&mut connection, member_id).await?;

    let pkce_verifier = PkceCodeVerifier::new(request_data.pkce_verifier);

    let oauth_http_client = oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| HttpClientFailed)?;

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| HttpClientFailed)?;

    let token = match client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&oauth_http_client)
        .await
    {
        Ok(token) => Ok(token),
        Err(oauth2::RequestTokenError::ServerResponse(response)) => Err(UserError::DiscordAuthInvalidTokenExchange(response)),
        Err(_) => Err(UserError::DiscordAuthTokenExchangeFailed),
    }?;

    // Check for a pending player claim from this user
    let player_claim = match PlayerClaim::by_user(member_id, &mut connection).await? {
        Some(claim) if !claim.verified => claim,
        _ => return Err(UserError::DiscordAuthNoUnverifiedPlayerClaim.into()),
    };
    let discord_connections_result = http_client
        .get("https://discord.com/api/users/@me/connections")
        .bearer_auth(token.access_token().secret())
        .send()
        .await;

    let connections: Vec<DiscordConnection> = match discord_connections_result {
        Ok(res) => match res.json().await {
            Ok(u) => Ok(u),
            Err(_) => Err(CoreError::UnprocessableEntity.into()),
        },
        Err(_) => Err(UserError::HttpClientFailed),
    }?;

    let linked_youtube_ids = connections
        .iter()
        .filter_map(|conn| {
            if conn.verified && conn.connection_type == "youtube" {
                Some(conn.id.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let player_channels = PlayerChannels::channels_for_player(&player_claim.player.id, &mut connection).await?;

    if !linked_youtube_ids.iter().any(|id| player_channels.channel_ids.contains(id)) {
        return Err(UserError::DiscordAuthNoMatchingYouTubeConnections.into());
    }

    match PlayerClaim::verified_claim_on(player_claim.player.id, &mut connection).await? {
        Some(_) => return Err(DemonlistError::AlreadyClaimed.into()),
        None => {
            PlayerClaim {
                user_id: member_id,
                player_id: player_claim.player.id,
                verified: player_claim.verified,
                lock_submissions: player_claim.lock_submissions,
            }
            .set_verified(true, &mut connection)
            .await?
        },
    }

    connection.commit().await.map_err(CoreError::from)?;

    // rocket's Redirect wont work properly here because of our auth cookie rules
    // https://stackoverflow.com/questions/42216700/how-can-i-redirect-after-oauth2-with-samesite-strict-and-still-get-my-cookies
    Ok(Response2::new((ContentType::HTML, ())).with_header("Refresh", "0; url=/account/"))
}
