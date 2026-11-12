-- Add up migration script here
ALTER TABLE members
    ADD COLUMN discord_id VARCHAR(64),
    ADD COLUMN discord_username CITEXT;

CREATE TABLE discord_oauth_requests (
    csrf_token VARCHAR(255) PRIMARY KEY NOT NULL,
    pkce_verifier VARCHAR(255) NOT NULL,
    member_id INTEGER NOT NULL REFERENCES members(member_id) ON DELETE CASCADE
);
