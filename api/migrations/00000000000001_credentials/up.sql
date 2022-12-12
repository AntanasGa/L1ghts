-- Your SQL goes here
CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  user_name TEXT NOT NULL,
  pass TEXT,
  recovery_key TEXT,
  recovery_expires TIMESTAMP
);
