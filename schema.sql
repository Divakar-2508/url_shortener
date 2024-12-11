BEGIN;

CREATE TABLE IF NOT EXISTS urls(
    id INTEGER PRIMARY KEY,
    url TEXT NOT NULL,
    short_code TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    access_count INTEGER DEFAULT 0
);

COMMIT;
