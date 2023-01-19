-- Your SQL goes here
CREATE TABLE devices (
    id INTEGER PRIMARY KEY NOT NULL,
    adr INTEGER UNIQUE NOT NULL,
    endpoint_count INTEGER NOT NULL
);
