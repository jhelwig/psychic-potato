SELECT strings.id AS 'id!: Uuid',
  strings.string_date AS 'string_date!: NaiveDate',
  strings.string_name AS 'string_name!: String',
  strings.target AS 'target!: String',
  strings.distance AS 'distance!: String',
  strings.score AS 'score!: sqlx::types::Json<StringScore>',
  strings.export_id AS 'export_id!: Uuid',
  strings.shooter_id AS 'shooter_id: Uuid',
  strings.class_id AS 'class_id: Uuid'
FROM "strings"
  INNER JOIN exports ON exports.id = strings.export_id
WHERE exports.match_id = $1
  AND strings.id = $2;
