-- Add migration script here

-- SQLite schema for desktop app using Better Auth design

PRAGMA foreign_keys = ON;

-- User table
CREATE TABLE IF NOT EXISTS user (
    id TEXT PRIMARY KEY,
    name TEXT,
    email TEXT UNIQUE,
    emailVerified BOOLEAN DEFAULT 0,
    image TEXT,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Session table
CREATE TABLE IF NOT EXISTS session (
    id TEXT PRIMARY KEY,
    userId TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    expiresAt TIMESTAMP NOT NULL,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE
);

-- Account table
CREATE TABLE IF NOT EXISTS account (
    id TEXT PRIMARY KEY,
    userId TEXT NOT NULL,
    accountId TEXT NOT NULL,
    providerId TEXT NOT NULL,
    accessToken TEXT,
    refreshToken TEXT,
    accessTokenExpiresAt TIMESTAMP,
    refreshTokenExpiresAt TIMESTAMP,
    scope TEXT,
    idToken TEXT,
    password TEXT,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE,
    UNIQUE(providerId, accountId)
);

-- Verification table (for email verification, password reset, etc.)
CREATE TABLE IF NOT EXISTS verification (
    id TEXT PRIMARY KEY,
    identifier TEXT NOT NULL,
    value TEXT NOT NULL,
    expiresAt TIMESTAMP NOT NULL,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- App settings table to track current user and other app configurations
CREATE TABLE IF NOT EXISTS app_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    currentUserId TEXT,
    theme TEXT DEFAULT 'light',
    language TEXT DEFAULT 'en',
    lastLogin TIMESTAMP,
    autoLogin BOOLEAN DEFAULT 0,
    FOREIGN KEY (currentUserId) REFERENCES user(id) ON DELETE SET NULL
);

-- Triggers to update the updatedAt timestamp
CREATE TRIGGER IF NOT EXISTS user_update_timestamp
AFTER UPDATE ON user
BEGIN
    UPDATE user SET updatedAt = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS session_update_timestamp
AFTER UPDATE ON session
BEGIN
    UPDATE session SET updatedAt = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS account_update_timestamp
AFTER UPDATE ON account
BEGIN
    UPDATE account SET updatedAt = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS verification_update_timestamp
AFTER UPDATE ON verification
BEGIN
    UPDATE verification SET updatedAt = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Insert initial app settings record
INSERT OR IGNORE INTO app_settings (id, theme, language, autoLogin) 
VALUES (1, 'light', 'en', 0);
