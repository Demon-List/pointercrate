use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use serde::Deserialize;
use sqlx::PgConnection;

use crate::config::{discord_client_id, discord_client_secret, discord_redirect_uri};
use crate::error::Result;

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct DiscordConnection {
    #[serde(rename(deserialize = "type"))]
    pub connection_type: String,
    pub revoked: bool,
    pub verified: bool,
    pub visibility: bool
}

type BasicClientWithToken = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct DiscordAuthRequestData {
    pub csrf_token: String,
    pub pkce_verifier: String,
}

pub fn get_discord_client() -> BasicClientWithToken {
    let client_id = oauth2::ClientId::new(discord_client_id());
    let client_secret = oauth2::ClientSecret::new(discord_client_secret());
    let auth_url =
        oauth2::AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = oauth2::TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).expect("Invalid token endpoint URL");
    let redirect_url = oauth2::RedirectUrl::new(discord_redirect_uri()).expect("Invalid redirect URL");

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
}

impl DiscordAuthRequestData {
    pub fn build(csrf_token: String, pkce_verifier: String) -> Self {
        Self { csrf_token, pkce_verifier }
    }
    pub async fn create(connection: &mut PgConnection, data: Self, member_id: i32) -> Result<()> {
        sqlx::query!(
            "INSERT INTO discord_oauth_requests (csrf_token, pkce_verifier, member_id) VALUES ($1, $2, $3);",
            data.csrf_token,
            data.pkce_verifier,
            member_id
        )
        .execute(connection)
        .await?;

        Ok(())
    }
    pub async fn take(connection: &mut PgConnection, csrf_token: &String) -> Result<(Self, i32)> {
        let row = sqlx::query!(
            "DELETE FROM discord_oauth_requests WHERE csrf_token = $1 RETURNING csrf_token, pkce_verifier, member_id",
            csrf_token
        )
        .fetch_one(connection)
        .await?;

        Ok((
            Self {
                pkce_verifier: row.pkce_verifier,
                csrf_token: row.csrf_token,
            },
            row.member_id,
        ))
    }
}

impl DiscordUser {
    pub async fn set_member_discord_data(connection: &mut PgConnection, member_id: i32, data: Self) -> Result<()> {
        sqlx::query!(
            "UPDATE members SET discord_id = $1, discord_username = $2 WHERE member_id = $3",
            data.id,
            data.username,
            member_id
        )
        .execute(connection)
        .await?;

        Ok(())
    }
}
