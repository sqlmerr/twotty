
CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY NOT NULl,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULl
);
