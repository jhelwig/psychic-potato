UPDATE strings
SET class_id = $4
FROM exports
  INNER JOIN matches ON matches.id = exports.match_id
WHERE exports.id = strings.export_id
  AND matches.league_id = $1
  AND matches.id = $2
  AND strings.id = $3;
