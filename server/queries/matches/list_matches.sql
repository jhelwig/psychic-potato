SELECT id AS "id!: uuid::Uuid",
  match_name AS "name!: String",
  event_date AS "event_date!: NaiveDate"
FROM matches
ORDER BY event_date
