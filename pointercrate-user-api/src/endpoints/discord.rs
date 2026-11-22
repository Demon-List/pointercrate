use std::net::IpAddr;

use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse as _};
use pointercrate_core::{error::CoreError, pool::PointercratePool};
use pointercrate_core_api::error::Result;
use pointercrate_core_macros::localized;
use pointercrate_demonlist::player::{channel::PlayerChannels, claim::PlayerClaim};
use pointercrate_user::{
    auth::ApiToken,
    discord::{get_discord_client, DiscordAuthRequestData, DiscordConnection},
    error::UserError::{self, HttpClientFailed},
};
use rocket::{response::Redirect, State};

use crate::{auth::Auth, ratelimits::UserRatelimits};

#[localized]
#[rocket::get("/discord/")]
pub async fn link_discord(mut auth: Auth<ApiToken>, ratelimits: &State<UserRatelimits>, ip: IpAddr) -> Result<Redirect> {
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
        .add_scopes(vec![Scope::new("identify".to_string()), Scope::new("connections".to_string())])
        .set_pkce_challenge(pkce_challenge)
        .url();

    DiscordAuthRequestData::create(
        &mut auth.connection,
        DiscordAuthRequestData::build(csrf_token.into_secret(), pkce_verifier.into_secret()),
        member_id,
    )
    .await?;
    auth.commit().await?;

    Ok(Redirect::to(auth_url.to_string()))
}

#[localized]
#[rocket::get("/discord/callback/?<code>&<state>")]
pub async fn link_discord_callback(
    mut auth: Auth<ApiToken>, pool: &State<PointercratePool>, code: String, state: String,
) -> Result<Redirect> {
    let mut pool_connection = pool.connection().await?;
    let client = get_discord_client();
    let code = AuthorizationCode::new(code);

    // This should not get rolled back on error
    let request_data = DiscordAuthRequestData::take(&mut pool_connection, &state, auth.user.user().id).await?;

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
    let player_claim = match PlayerClaim::by_user(auth.user.user().id, &mut auth.connection).await? {
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

    let player_channels = PlayerChannels::channels_for_player(&player_claim.player.id, &mut auth.connection).await?;

    if !linked_youtube_ids.iter().any(|id| player_channels.all_channel_ids.contains(id)) {
        return Err(UserError::DiscordAuthNoMatchingYouTubeConnections.into());
    }
    sqlx::query!("UPDATE player_claims SET verified = TRUE WHERE member_id = $1", auth.user.user().id)
        .execute(&mut *auth.connection)
        .await
        .map_err(UserError::from)?;

    auth.commit().await?;

    Ok(Redirect::to("/account"))
}
