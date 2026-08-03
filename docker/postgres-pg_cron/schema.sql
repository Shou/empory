
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
    created_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);
CREATE INDEX timeline_feed_sorted ON timeline (user_id, created_at DESC);

-- NOTE we avoid a completed_at by just deleting entries when finished
-- if we need historical jobs we can add a separate table for that
CREATE TABLE IF NOT EXISTS jobs (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    type TEXT NOT NULL,
    payload JSONB NOT NULL,

    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    available_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    locked_at TIMESTAMPTZ,

    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT
);
CREATE INDEX jobs_ready_idx
ON jobs (type, available_at, id)
WHERE locked_at IS NULL;

-- TODO partition?
CREATE TABLE IF NOT EXISTS likes (
    user_id UUID NOT NULL,
    post_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

-- TODO: CREATE TABLE notifications ( ... )

CREATE TABLE IF NOT EXISTS avatars (
    id SERIAL PRIMARY KEY,
    user_id UUID UNIQUE,
    url VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (timezone('utc', now())),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- CREATE EXTENSION IF NOT EXISTS pg_cron;
