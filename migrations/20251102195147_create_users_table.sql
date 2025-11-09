DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS logs;


CREATE TABLE users (
    public_key VARCHAR(64) PRIMARY KEY,
    username VARCHAR(50),
    rewards TEXT[],
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    banned BOOLEAN NOT NULL DEFAULT FALSE,
    ban_reason TEXT
);

CREATE TABLE logs (
    id SERIAL PRIMARY KEY,
    user_public_key VARCHAR(32) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    source VARCHAR(32) NOT NULL,
    error_code VARCHAR(64),
    message TEXT NOT NULL,
    criticality BOOLEAN DEFAULT FALSE,
    context JSONB
);