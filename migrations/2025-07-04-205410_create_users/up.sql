-- Your SQL goes here

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    real_name TEXT,
    bio TEXT
)