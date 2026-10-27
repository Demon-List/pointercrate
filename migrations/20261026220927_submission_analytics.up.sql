CREATE TABLE submission_analytics (
    id SERIAL PRIMARY KEY,
    time TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'utc') NOT NULL,
    list CITEXT NOT NULL,
    record_id INTEGER -- no need to enforce fkey here, would just be source of unnecessary hassle
);