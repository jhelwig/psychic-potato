SELECT id AS "id!: Uuid",
  league_name AS "name!: String",
  description AS "description: String",
  start_date AS "start_date: NaiveDate",
  end_date AS "end_date: NaiveDate",
  created_at AS "created_at!: DateTime<Utc>"
FROM leagues
WHERE id = $1;
