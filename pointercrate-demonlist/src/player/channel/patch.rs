use serde::Deserialize;
use sqlx::PgConnection;

use crate::{error::Result, player::channel::PlayerChannels};

#[derive(Debug, Deserialize)]
pub struct PatchPlayerChannels {
    channel_id: String,
    validity: bool,
}

impl PlayerChannels {
    pub async fn set_player_channel_validity(player_id: i32, data: &PatchPlayerChannels, connection: &mut PgConnection) -> Result<()> {
        sqlx::query("UPDATE player_youtube_channels SET valid = $1 WHERE player_id = $2 AND channel_id = $3")
            .bind(data.validity)
            .bind(player_id)
            .bind(&data.channel_id)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }
}
