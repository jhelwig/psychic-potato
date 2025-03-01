SELECT id AS "id!: uuid::Uuid",
  league_name AS "name!: String",
  created_at AS "created_at!: DateTime<Utc>"
FROM leagues
ORDER BY league_name;
