-- Your SQL goes here
CREATE TABLE credentials (
  id INTEGER NOT NULL PRIMARY KEY,
  user_name TEXT NOT NULL,
  pass TEXT,
  recovery_key TEXT,
  recovery_expires TIMESTAMP
);
