SELECT exports.id AS "id!: uuid::Uuid",
  exports.file_name AS "file_name!: String",
  exports.generated_date AS "generated_date!: chrono::NaiveDate",
  exports.string_count AS "string_count!: i32",
  exports.string_date AS "string_date!: chrono::NaiveDate",
  exports.match_id AS "match_id!: uuid::Uuid"
FROM exports
  INNER JOIN matches ON exports.match_id = matches.id
WHERE matches.league_id = $1
  AND matches.id = $2
  AND exports.id = $3
