SELECT id AS "id!: Uuid",
  shooter_name AS "name!: String",
  default_class_id AS "default_class_id?: Uuid"
FROM shooters
WHERE id = $1;
