CREATE TABLE player_youtube_channels (
    player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    channel_id TEXT NOT NULL,
    valid BOOLEAN NOT NULL DEFAULT TRUE,

    PRIMARY KEY (player_id, channel_id)
);
