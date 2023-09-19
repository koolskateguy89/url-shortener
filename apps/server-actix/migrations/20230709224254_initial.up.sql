-- Add up migration script here
CREATE TABLE users (
  username TEXT PRIMARY KEY,
  password TEXT NOT NULL
);

CREATE TABLE urls (
  id         VARCHAR(6)  PRIMARY KEY,
  url        TEXT        NOT NULL,
  username   TEXT        DEFAULT NULL
                         REFERENCES users
                         ON DELETE SET NULL ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  -- Can't use NULLS NOT DISTINCT cos postgres 11
  UNIQUE(url, username)
);

CREATE TABLE lengthen_logs (
  id   VARCHAR(6)  REFERENCES urls ON DELETE CASCADE ON UPDATE CASCADE,
  date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY(id, date)
);
