DROP TABLE IF EXISTS shots;
DROP TABLE IF EXISTS shot_strings;
DROP TABLE IF EXISTS matches;

CREATE TABLE matches (
  id TEXT PRIMARY KEY,
  NAME TEXT,
  DATE TEXT
);

CREATE TABLE shot_strings (
  id TEXT PRIMARY KEY,
  match_id TEXT,
  DATE TEXT,
  NAME TEXT,
  target TEXT,
  distance TEXT,
  score TEXT,
  metrics TEXT,
  FOREIGN KEY(match_id) REFERENCES matches(id)
);

CREATE TABLE shots (
  id TEXT PRIMARY KEY,
  shot_string_id TEXT,
  shot_string TEXT,
  shot_id TEXT,
  tags TEXT,
  score TEXT,
  position TEXT,
  velocity TEXT,
  yaw TEXT,
  pitch TEXT,
  quality TEXT,
  FOREIGN KEY(shot_string_id) REFERENCES shot_strings(id)
);
