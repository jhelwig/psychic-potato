SELECT id AS 'id!: Uuid',
  username AS 'username!: String',
  password_hash AS 'password_hash!: String'
FROM users
WHERE username = $1;
