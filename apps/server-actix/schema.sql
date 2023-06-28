DROP TABLE IF EXISTS urls CASCADE;
DROP TABLE IF EXISTS lengthen_logs;
DROP TABLE IF EXISTS users;

CREATE TABLE urls (
  id  VARCHAR(6) PRIMARY KEY,
  url TEXT       NOT NULL UNIQUE
);

INSERT INTO
  urls(id, url)
VALUES
  ('abc123', 'https://www.google.com/')
;

CREATE TABLE lengthen_logs (
  id         VARCHAR(6) REFERENCES urls
                          ON DELETE CASCADE
                          ON UPDATE CASCADE,
  created_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,

  -- TODO?: take created_at out of PK
  PRIMARY KEY(id, created_at)
);

INSERT INTO
  lengthen_logs(id)
VALUES
  ('abc123')
;

-- TODO: best practice for text type?
CREATE TABLE users (
  username TEXT PRIMARY KEY,
  password TEXT NOT NULL
);

INSERT INTO
  users(username, password)
VALUES
  ('test', 'testpw')
;
