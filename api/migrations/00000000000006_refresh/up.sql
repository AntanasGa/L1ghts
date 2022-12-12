-- Your SQL goes here
CREATE TABLE credential_refresh (
    id SERIAL PRIMARY KEY NOT NULL,
    credential_id INTEGER NOT NULL REFERENCES credentials(id),
    token TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP NOT NULL
);
