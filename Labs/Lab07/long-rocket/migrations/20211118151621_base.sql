-- Add migration script here
CREATE TABLE IF NOT EXISTS blogs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	timestamp TEXT NOT NULL,
	text TEXT NOT NULL
);
