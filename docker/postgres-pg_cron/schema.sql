
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    last_seen_at TIMESTAMPTZ DEFAULT (timezone('utc', now()))
);

CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    token_hash BYTEA NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (now() + interval '30 days'),
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    revoked_at TIMESTAMPTZ,
    ip_address INET,
    user_agent TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- TODO partition
CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    public_id TEXT NOT NULL DEFAULT nanoid(size => 18),
    user_id UUID NOT NULL,
    content TEXT NOT NULL,
    like_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS follows (
    user_id UUID NOT NULL,
    followed_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    PRIMARY KEY (user_id, followed_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (followed_id) REFERENCES users(id) ON DELETE CASCADE
);

-- TODO partition
CREATE TABLE IF NOT EXISTS timeline (
    user_id UUID NOT NULL,
    post_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);
CREATE INDEX timeline_feed_sorted ON timeline (user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS timeline_jobs (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id UUID NOT NULL,
    post_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    queued_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

-- TODO partition
CREATE TABLE IF NOT EXISTS likes (
    user_id UUID NOT NULL,
    post_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

-- TODO create table notifications

CREATE TABLE IF NOT EXISTS avatars (
    id SERIAL PRIMARY KEY,
    user_id UUID UNIQUE,
    url VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

DROP MATERIALIZED VIEW IF EXISTS feed_posts;
CREATE MATERIALIZED VIEW feed_posts AS
SELECT
    f.user_id AS feed_owner_id,
    p.id AS post_id,
    p.user_id AS author_id,
    u.username AS author_username,
    a.url AS avatar_url,
    p.content AS post_content,
    p.created_at AS post_created_at,
    l.like_count AS like_count
FROM follows f
JOIN posts p ON p.user_id = f.followed_id
JOIN users u ON u.id = p.user_id
LEFT JOIN avatars a ON a.user_id = u.id
LEFT JOIN (
    SELECT post_id, COUNT(*) AS like_count FROM likes GROUP BY post_id
) l on l.post_id = p.id;
CREATE UNIQUE INDEX feed_posts_pk ON feed_posts (feed_owner_id, post_id);
CREATE INDEX feed_posts_sorted_idx ON feed_posts (feed_owner_id, post_created_at DESC, post_id DESC);

CREATE EXTENSION IF NOT EXISTS pg_cron;

SELECT cron.schedule(
    'refresh-feeds',
    '*/1 * * * *',
    'REFRESH MATERIALIZED VIEW CONCURRENTLY feed_posts'
);
