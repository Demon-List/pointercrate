-- Add down migration script here
DROP TABLE discord_oauth_requests;
ALTER TABLE members DROP COLUMN discord_id, DROP COLUMN discord_username;
