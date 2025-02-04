DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS user_shot_strings;

CREATE TABLE users (
  id TEXT PRIMARY KEY,
  NAME TEXT,
  email TEXT UNIQUE
);

CREATE TABLE user_shot_strings (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  shot_string_id TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (shot_string_id) REFERENCES shot_strings(id)
);
