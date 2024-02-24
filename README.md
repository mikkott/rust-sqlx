# rust-sqlx

## sqlx-cli

### Install sqlx cli client
`$ cargo install sqlx-cli`

### Create sqlite database
`$ export DATABASE_URL="sqlite:db.sqlite3"`

`$ sqlx database create`

### Create/add schema
`$ sqlx migrate add users`

Add some SQL to initialize table schema in DB
```
cat <<EOT > $(ls migrations/*.sql)
-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 0
);
EOT
```

### un migrations to create table(s) in database
`$ sqlx migrate run`
