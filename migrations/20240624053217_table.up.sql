-- Add migration script here
CREATE TABLE IF NOT EXISTS Articles (
    slug text NOT NULL PRIMARY KEY,
    title text NOT NULL,
    description text NOT NULL,
    body text NOT NULL,
    -- author text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL default NOW(),
    updated_at TIMESTAMPTZ NOT NULL default NOW()
);

-- CREATE TABLE IF NOT EXISTS Users (
--     username text NOT NULL PRIMARY KEY,
--     email text NOT NULL UNIQUE,
--     password text NOT NULL,
--     bio text NULL,
--     image text NULL
-- );
