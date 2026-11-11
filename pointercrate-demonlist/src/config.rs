use pointercrate_core::util::from_env_or_default;

pub fn list_size() -> i16 {
    from_env_or_default("LIST_SIZE", 50)
}

pub fn extended_list_size() -> i16 {
    from_env_or_default("EXTENDED_LIST_SIZE", 100)
}

#[cfg(feature = "player_channels")]
pub fn youtube_api_key() -> String {
    std::env::var("YOUTUBE_API_KEY").expect("Missing YOUTUBE_API_KEY; did not register channel ID")
}
