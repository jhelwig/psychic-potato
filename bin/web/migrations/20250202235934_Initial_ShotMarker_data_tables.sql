DROP TABLE IF EXISTS shot;
DROP TABLE IF EXISTS shot_string;
DROP TABLE IF EXISTS match;

CREATE TABLE match (
  id TEXT PRIMARY KEY,
  NAME TEXT,
  DATE TEXT
);

CREATE TABLE shot_string (
  id TEXT PRIMARY KEY,
  match_id TEXT,
  DATE TEXT,
  NAME TEXT,
  target TEXT,
  distance TEXT,
  score TEXT,
  metrics TEXT,
  FOREIGN KEY(match_id) REFERENCES match(id)
);

CREATE TABLE shot (
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
  FOREIGN KEY(shot_string_id) REFERENCES shot_string(id)
);
