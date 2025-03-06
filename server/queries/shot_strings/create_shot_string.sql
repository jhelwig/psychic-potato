INSERT INTO strings (
    id,
    string_date,
    string_name,
    target,
    distance,
    score,
    export_id
  )
VALUES ($1, $2, $3, $4, $5, $6, $7);
