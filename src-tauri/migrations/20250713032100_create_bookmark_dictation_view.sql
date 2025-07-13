-- Add migration script here

PRAGMA foreign_keys = ON;

-- Create a view that joins bookmark and dictation tables
-- This allows querying both bookmark and dictation data together efficiently
-- Using UNION to simulate FULL OUTER JOIN since SQLite doesn't support it
CREATE VIEW IF NOT EXISTS bookmark_dictation_view AS
SELECT 
    b.userId,
    b.audioId,
    b.id as bookmark_id,
    b.bookmarkId,
    b.createdAt as bookmark_created_at,
    d.id as dictation_id,
    d.dictationId,
    d.createdAt as dictation_created_at
FROM bookmark b
LEFT JOIN dictation d ON b.userId = d.userId AND b.audioId = d.audioId

UNION

SELECT 
    d.userId,
    d.audioId,
    b.id as bookmark_id,
    b.bookmarkId,
    b.createdAt as bookmark_created_at,
    d.id as dictation_id,
    d.dictationId,
    d.createdAt as dictation_created_at
FROM dictation d
LEFT JOIN bookmark b ON d.userId = b.userId AND d.audioId = b.audioId
WHERE b.id IS NULL;