UPDATE shooters
SET default_class_id = $2
WHERE id = $1;
