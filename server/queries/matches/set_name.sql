UPDATE matches
SET match_name = $3
WHERE league_id = $1
  AND id = $2;
