INSERT INTO shots (
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
    string_id
  )
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
