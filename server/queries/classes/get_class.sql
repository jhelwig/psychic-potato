SELECT id AS 'id!: Uuid',
  class_name AS 'name!: String',
  description AS 'description: String',
  league_id AS 'league_id!: Uuid'
FROM classes
WHERE league_id = $1
  AND id = $2;
