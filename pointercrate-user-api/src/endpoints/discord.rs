use std::net::IpAddr;

use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};
use pointercrate_core::error::CoreError::InternalServerError;
use pointercrate_core_api::error::{ErrorResponder, Result};
use pointercrate_core_macros::localized;
use pointercrate_user::{
    auth::ApiToken,
    discord::{get_discord_client, DiscordAuthRequestData, DiscordUser, DiscordConnection},
};
use rocket::{http::Status, response::Redirect, State}

use crate::{auth::Auth, ratelimits::UserRatelimits};

#[localized]
#[rocket::get("/discord/")]
pub async fn link_discord(mut auth: Auth<ApiToken>, ratelimits: &State<UserRatelimits>, ip: IpAddr) -> Result<Redirect> {
    ratelimits.discord_link_attempts(ip)?;
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
        auth.user.into_user().id,
    )
    .await?;

    Ok(Redirect::to(auth_url.to_string()))
}

#[localized]
#[rocket::get("/discord/callback/?<code>&<state>")]
pub async fn link_discord_callback(mut auth: Auth<ApiToken>, code: String, state: String) -> Result<Status> {
    let client = get_discord_client();
    let code = AuthorizationCode::new(code);

    let (request_data, initiator_id) = DiscordAuthRequestData::take(&mut auth.connection, &state).await?;

    if initiator_id != auth.user.user().id {
        // return error
        unimplemented!()
    }

    let pkce_verifier = PkceCodeVerifier::new(request_data.pkce_verifier);

    let http_client = match reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none()).build() {
        Ok(client) => Ok(client),
        Err(e) => Err(InternalServerError),
    }?;

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    let discord_user_result = http_client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await;

    let discord_user: DiscordUser = match discord_user_result {
        Ok(res) => match res.json().await {
            Ok(u) => u,
            Err(_) => return Err(InternalServerError),
        },
        Err(_) => return Err(InternalServerError),
    };

    DiscordUser::set_member_discord_data(&mut auth.connection, initiator_id, discord_user)
        .await
        .expect("lol");

    let discord_connections_result = http_client
        .get("https://discord.com/api/users/@me/connections")
        .bearer_auth(token.access_token().secret())
        .send()
        .await;

    let discord_user: Vec<DiscordConnection> = match discord_connections_result {
        Ok(res) => match res.json().await {
            Ok(u) => u,
            Err(_) => return Err(InternalServerError),
        },
        Err(_) => return Err(InternalServerError),
    };

    todo!("needs to find pending player claim, validate connection links against player channels: `PlayerChannels::channels_for_player`, approve the claim if exists")
}
