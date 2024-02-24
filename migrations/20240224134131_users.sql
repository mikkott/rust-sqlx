-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 01
);
