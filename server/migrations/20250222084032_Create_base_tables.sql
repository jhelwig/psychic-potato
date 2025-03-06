CREATE TABLE leagues (
  id TEXT PRIMARY KEY,
  league_name TEXT,
  description TEXT,
  created_at TEXT,
  start_date TEXT,
  end_date TEXT
);

CREATE TABLE matches (
  id TEXT PRIMARY KEY,
  match_name TEXT,
  event_date TEXT,
  league_id TEXT REFERENCES leagues(id) ON
  DELETE CASCADE
);

CREATE TABLE shotmarker_export_files (
  id TEXT PRIMARY KEY,
  file_name TEXT,
  contents TEXT,
  uploaded_at TEXT,
  export_id TEXT REFERENCES exports(id) ON
  DELETE CASCADE
);

CREATE TABLE exports (
  id TEXT PRIMARY KEY,
  file_name TEXT,
  generated_date TEXT,
  string_count INTEGER,
  string_date TEXT,
  match_id TEXT REFERENCES matches(id) ON
  DELETE CASCADE
);

CREATE TABLE strings (
  id TEXT PRIMARY KEY,
  string_date TEXT,
  string_name TEXT,
  target TEXT,
  distance TEXT,
  score TEXT,
  export_id TEXT REFERENCES exports(id) ON
  DELETE CASCADE,
    shooter_id TEXT REFERENCES shooters(id) ON
  DELETE
  SET NULL,
    class_id TEXT REFERENCES classes(id) ON
  DELETE
  SET NULL
);

CREATE TABLE shots (
  id TEXT PRIMARY KEY,
  shot_time TEXT,
  shot_id TEXT,
  tags TEXT,
  score TEXT,
  position TEXT,
  velocity TEXT,
  yaw TEXT,
  pitch TEXT,
  quality TEXT,
  string_id TEXT REFERENCES strings(id) ON
  DELETE CASCADE
);

CREATE TABLE classes (
  id TEXT PRIMARY KEY,
  class_name TEXT,
  description TEXT,
  league_id TEXT REFERENCES leagues(id) ON
  DELETE CASCADE
);

CREATE TABLE shooters (
  id TEXT PRIMARY KEY,
  shooter_name TEXT,
  default_class_id TEXT REFERENCES classes(id) ON
  DELETE
  SET NULL
);
