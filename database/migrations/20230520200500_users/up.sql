CREATE TABLE users (
    id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(16) NOT NULL UNIQUE CONSTRAINT check_username CHECK (username ~* '^[a-z0-9_]{3,25}$'),
    password_hash TEXT NOT NULL,
    token_expires TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);
