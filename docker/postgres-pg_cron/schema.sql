
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_sessions (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    ip_address INET,
    user_agent TEXT
);

CREATE TABLE IF NOT EXISTS api_tokens (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now()))
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now()))
);

CREATE TABLE IF NOT EXISTS follows (
    user_id UUID NOT NULL REFERENCES users(id),
    followed_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT (timezone('utf', now())),
    PRIMARY KEY (user_id, followed_id)
);

CREATE TABLE IF NOT EXISTS avatars (
    id SERIAL PRIMARY KEY,
    user_id UUID UNIQUE REFERENCES users(id),
    url VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utf', now()))
);

CREATE MATERIALIZED VIEW IF NOT EXISTS user_feeds AS
SELECT
    f.user_id AS feed_owner_id,
    p.id AS post_id,
    p.user_id AS author_id,
    u.username AS author_username,
--    a.url AS avatar_url,
    p.content AS post_content,
    p.created_at AS post_created_at
FROM follows f
JOIN posts p ON p.user_id = f.followed_id
JOIN users u ON u.id = p.user_id;
--JOIN avatars a ON a.user_id = u.user_id;

CREATE EXTENSION IF NOT EXISTS pg_cron;

SELECT cron.schedule(
    'refresh-feeds',
    '*/1 * * * *',
    'REFRESH MATERIALIZED VIEW CONCURRENTLY user_feeds'
);