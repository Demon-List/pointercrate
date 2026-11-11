use sqlx::{PgConnection, Row};

use crate::{error::Result, player::channel::PlayerChannels};

impl PlayerChannels {
    pub async fn channels_for_player(player_id: &i32, connection: &mut PgConnection) -> Result<PlayerChannels> {
        let rows = sqlx::query("SELECT player_id, channel_id, valid FROM player_youtube_channels WHERE player_id = $1")
            .bind(player_id)
            .fetch_all(&mut *connection)
            .await?;

        Ok(PlayerChannels {
            channel_ids: rows
                .iter()
                .filter(|r| r.get("valid"))
                .map(|r| r.get("channel_id"))
                .collect::<Vec<String>>(),
            all_channel_ids: rows.iter().map(|r| r.get("channel_id")).collect::<Vec<String>>(),
        })
    }
}
