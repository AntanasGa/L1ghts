-- Your SQL goes here
CREATE TABLE devices (
    id SERIAL PRIMARY KEY NOT NULL,
    adr INTEGER UNIQUE NOT NULL,
    pairs_of INTEGER NOT NULL,
    endpoint_count INTEGER NOT NULL
);
