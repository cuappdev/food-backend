-- Add migration script here
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    user_id INTEGER NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    session_token TEXT NOT NULL
);