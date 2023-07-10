-- Add up migration script here
CREATE TABLE urls (
  id  VARCHAR(6) PRIMARY KEY,
  url TEXT       NOT NULL UNIQUE
);

CREATE TABLE lengthen_logs (
  id         VARCHAR(6)  REFERENCES urls
                            ON DELETE CASCADE
                            ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY(id, created_at)
);

CREATE TABLE users (
  username TEXT PRIMARY KEY,
  password TEXT NOT NULL
);
