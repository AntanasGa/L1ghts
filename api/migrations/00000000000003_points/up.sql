-- Your SQL goes here
CREATE TABLE points (
    id SERIAL PRIMARY KEY NOT NULL,
    device_id INTEGER NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    device_position INTEGER NOT NULL,
    val INTEGER NOT NULL DEFAULT 0 CHECK (val > -1),
    width REAL NOT NULL,
    height REAL NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    rotation REAL NOT NULL,
    watts REAL NOT NULL,
    active BOOLEAN NOT NULL,
    tag TEXT
);
