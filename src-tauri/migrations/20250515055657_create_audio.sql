-- Add migration script here

PRAGMA foreign_keys = ON;
--
CREATE TABLE IF NOT EXISTS audio (
    id TEXT PRIMARY KEY,
    userId TEXT NOT NULL,
    title TEXT NOT NULL,
	  description TEXT,
		url TEXT NOT NULL,
	  thumbnail TEXT,
	  startTime INTEGER NOT NULL,
		endTime INTEGER NOT NULL,
	  provider TEXT NOT NULL,
	  tag TEXT,
		transcribe INTEGER DEFAULT 0 CHECK (transcribe IN (0, 1)),
    lastUsedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE
);
--
--  -- 'echo', 'listening', 'pronunciation', etc.
CREATE TABLE IF NOT EXISTS audio_exercise_types (
    audioId TEXT,
    exerciseType TEXT,
    PRIMARY KEY (audioId, exerciseType),
    FOREIGN KEY (audioId) REFERENCES audio(id) ON DELETE CASCADE
);
