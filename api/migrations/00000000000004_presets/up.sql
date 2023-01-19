-- Your SQL goes here
CREATE TABLE presets (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES credentials(id) ON DELETE CASCADE,
    preset_name TEXT NOT NULL,
    favorite BOOLEAN NOT NULL,
    active BOOLEAN NOT NULL,
    icon TEXT
);
