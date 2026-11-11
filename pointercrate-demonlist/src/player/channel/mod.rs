use serde::Serialize;
use sqlx::prelude::FromRow;

pub mod get;
pub mod patch;
pub mod post;

#[derive(Debug, Serialize, FromRow)]
pub struct PlayerChannels {
    pub channel_ids: Vec<String>,

    /// Include invalidated channel IDs
    pub all_channel_ids: Vec<String>,
}
