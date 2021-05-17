CREATE TABLE IF NOT EXISTS commands(
    id INTEGER PRIMARY KEY,
    last_time_used INTEGER,
    command TEXT UNIQUE
);