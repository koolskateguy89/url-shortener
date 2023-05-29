-- this is default
DROP TABLE IF EXISTS todos;

CREATE TABLE todos (
  id serial PRIMARY KEY,
  note TEXT NOT NULL
);

-- TODO
CREATE TABLE urls (
  id VARCHAR(6) PRIMARY KEY,
  url TEXT NOT NULL
);
