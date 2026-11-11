use serde::Deserialize;
use sqlx::{pool::PoolConnection, PgConnection, Postgres};
use url::Url;

use crate::{error::Result, player::channel::PlayerChannels};

#[derive(Debug, Deserialize)]
pub struct PostPlayerChannel {
    channel_id: String,
}

impl PlayerChannels {
    /// this should be executed in a background task !!!
    /// it's safe to hold a db connection throughout all this since this can only be triggered by staff actions
    pub async fn add_channel_from_video(player_id: i32, video: String, mut connection: PoolConnection<Postgres>) {
        let url = match crate::video::validate(&video) {
            Ok(video) => Url::parse(&video).unwrap(), // safe cuz url is already validated in `crate::video::validate`
            Err(_) => {
                log::error!("Tried to add channel for invalid video {}", video);
                return;
            },
        };
        if url.domain().is_some_and(|host| host != "www.youtube.com") {
            log::warn!("Tried to add channel from video '{}', but host was not recognized", video);
            return;
        }

        if let Some(video_id) = url
            .query_pairs()
            .find_map(|(key, value)| if key == "v" { Some(value) } else { None })
        {
            let video_id: String = video_id.chars().take(11).collect();
            let data: serde_json::Value = reqwest::get(format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}", video_id, crate::config::youtube_api_key()))
                .await
                // here we pray that google's trillion dollar api works and we arent doing anything wrong
                .unwrap()
                .json()
                .await
                .unwrap();

            if let Err(err) = PlayerChannels::add_player_channel(
                player_id,
                &PostPlayerChannel {
                    channel_id: data["items"][0]["snippet"]["channelId"].as_str().unwrap().to_string(),
                },
                &mut *connection,
            )
            .await
            {
                log::error!("Failed to add player channel, {:#?}", err);
            }
        }
    }

    pub async fn add_player_channel(player_id: i32, data: &PostPlayerChannel, connection: &mut PgConnection) -> Result<()> {
        sqlx::query("INSERT INTO player_youtube_channels (player_id, channel_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(player_id)
            .bind(&data.channel_id)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }
}
