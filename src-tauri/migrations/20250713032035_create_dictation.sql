-- Add migration script here

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS dictation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    userId TEXT NOT NULL,
    audioId TEXT NOT NULL,
    dictationId INTEGER NOT NULL,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (audioId) REFERENCES audio(id) ON DELETE CASCADE,
    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE,
    UNIQUE(dictationId , audioId)
);
