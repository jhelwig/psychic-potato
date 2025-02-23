CREATE TABLE matches (id, match_name, event_date);

CREATE TABLE exports (
  id,
  generated_date,
  string_count,
  string_date,
  match_id,
  FOREIGN KEY (match_id) REFERENCES matches(id)
);

CREATE TABLE strings (
  id,
  string_date,
  string_name,
  target,
  distance,
  score,
  export_id,
  FOREIGN KEY (export_id) REFERENCES exports(id)
);

CREATE TABLE shots (
  id,
  shot_time,
  shot_id,
  tags,
  score,
  position,
  velocity,
  yaw,
  pitch,
  quality,
  string_id,
  FOREIGN KEY (string_id) REFERENCES strings(id)
)
