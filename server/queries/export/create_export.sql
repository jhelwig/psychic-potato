INSERT INTO exports (
    id,
    file_name,
    generated_date,
    string_count,
    string_date,
    match_id
  )
VALUES ($1, $2, $3, $4, $5, $6);
