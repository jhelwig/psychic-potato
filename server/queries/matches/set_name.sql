UPDATE matches
SET match_name = $2
WHERE id = $1;
