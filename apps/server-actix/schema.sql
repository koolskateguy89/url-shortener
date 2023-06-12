drop table if exists urls;

CREATE TABLE urls (
  id VARCHAR(6) PRIMARY KEY,
  url TEXT NOT NULL UNIQUE
);

INSERT INTO urls(id, url)
VALUES ('abc123', 'https://www.google.com/');
