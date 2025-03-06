INSERT INTO shotmarker_export_files (
    id,
    file_name,
    contents,
    uploaded_at,
    export_id
  )
VALUES ($1, $2, $3, $4, $5);
