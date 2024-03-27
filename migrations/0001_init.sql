CREATE TABLE
  users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    signature CHAR(20) NULL
  );

CREATE TYPE REGION AS ENUM ('EUW', 'EUNE', 'NA', 'KR');

CREATE TABLE
  account (
    id SERIAL PRIMARY KEY,
    in_game_name VARCHAR(16) NOT NULL,
    region REGION NOT NULL,
    tag VARCHAR(4) NOT NULL
  );

CREATE TABLE
  user_account (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users (id),
    account_id INT REFERENCES account (id)
  );

CREATE TYPE SCORELINE AS (
  champion_id SMALLINT,
  kills SMALLINT,
  deaths SMALLINT,
  assits SMALLINT,
  cs SMALLINT
);

CREATE TYPE TEAM AS (
  top SCORELINE,
  jungle SCORELINE,
  mid SCORELINE,
  bot SCORELINE,
  support SCORELINE
);

CREATE TYPE GAME_INFO AS (blue TEAM, red TEAM);

CREATE TABLE
  game (
    id SERIAL PRIMARY KEY,
    game_info GAME_INFO NOT NULL,
    played TIME
    WITH
      TIME ZONE NOT NULL
  );

CREATE TYPE GAME_SIDE AS ENUM ('blue', 'red');

CREATE TYPE GAME_ROLE AS ENUM ('top', 'jungle', 'mid', 'bottom', 'support');

CREATE TABLE
  note (
    id SERIAL PRIMARY KEY,
    note TEXT NULL,
    author_id INT REFERENCES users (id) NOT NULL,
    game_id INT REFERENCES game (id) NOT NULL,
    game_side GAME_SIDE NOT NULL,
    game_role GAME_ROLE NOT NULL
  );