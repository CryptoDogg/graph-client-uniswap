-- Your SQL goes here
CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  address VARCHAR NOT NULL,
  symbol TEXT NOT NULL
)