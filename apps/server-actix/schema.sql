DROP TABLE IF EXISTS urls CASCADE;
DROP TABLE IF EXISTS lengthen_logs;

CREATE TABLE urls (
  id  VARCHAR(6) PRIMARY KEY,
  url TEXT       NOT NULL UNIQUE
);

INSERT INTO urls(id, url)
VALUES ('abc123', 'https://www.google.com/');

-- TODO: users

CREATE TABLE lengthen_logs (
  id         VARCHAR(6) REFERENCES urls
                          ON DELETE CASCADE
                          ON UPDATE CASCADE,
  created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY(id, created_at)
);

INSERT INTO lengthen_logs(id)
VALUES ('abc123');
