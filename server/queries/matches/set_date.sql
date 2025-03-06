UPDATE matches
SET event_date = $3
WHERE league_id = $1
  AND id = $2;
