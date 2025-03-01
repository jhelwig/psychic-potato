SELECT id AS "id!: Uuid",
  league_name AS "name!: String",
  created_at AS "created_at!: DateTime<Utc>"
FROM leagues
WHERE id = $1;
