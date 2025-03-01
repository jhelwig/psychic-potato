SELECT id AS "id!: uuid::Uuid",
  file_name AS "file_name!: String",
  generated_date AS "generated_date!: chrono::NaiveDate",
  string_count AS "string_count!: i32",
  string_date AS "string_date!: chrono::NaiveDate",
  match_id AS "match_id!: uuid::Uuid"
FROM exports
ORDER BY generated_date
