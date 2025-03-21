SELECT shots.id AS "id!: Uuid",
  shots.shot_time AS "shot_time!: NaiveTime",
  shots.shot_id AS "shot_id!: String",
  shots.tags AS "tags!: String",
  shots.score AS "score!: sqlx::types::Json<ShotScore>",
  shots.position AS "position!: sqlx::types::Json<ShotPosition>",
  shots.velocity AS "velocity!: sqlx::types::Json<ShotVelocity>",
  shots.yaw AS "yaw!: f64",
  shots.pitch AS "pitch!: f64",
  shots.quality AS "quality?: String",
  shots.string_id AS "shot_string_id!: Uuid"
FROM shots
  INNER JOIN strings ON strings.id = shots.string_id
  INNER JOIN exports ON exports.id = strings.export_id
  INNER JOIN matches ON matches.id = exports.match_id
WHERE matches.league_id = $1
  AND matches.id = $2
  AND strings.id = $3
ORDER BY shots.shot_time;
