UPDATE matches
SET event_date = $2
WHERE id = $1;
