
-- only run if it's the first time and users dont exist (i.e. avoid spamming posts)
BEGIN;

INSERT INTO users (id, username, email, password_hash)
VALUES
    ('019f6025-bf1d-7a19-a3c4-1c9eb0688f01', 'alice', 'alice@example.com', '$2a$10$fakehash'),
    ('019f6025-e32f-7256-8c79-0a92e97ea681', 'bob', 'bob@example.com', '$2a$10$fakehash'),
    ('019f6026-007b-7ab2-a28a-fc86f98e362d', 'charlize', 'charlize@example.com', '$2a$10$fakehash'),
    ('019fb7dd-b1d4-73b2-8a1d-b25bf7fbf207', 'admin', 'admin@example.com', '$2b$12$H7y1jaQXjU8r4M7g6kBUluZHjMiEsRMpcy6mG3HgzD./nJ/xgHNHG');

INSERT INTO posts (user_id, content)
VALUES
    ('019f6025-bf1d-7a19-a3c4-1c9eb0688f01', 'Hello world! This is my first post.'),
    ('019f6025-bf1d-7a19-a3c4-1c9eb0688f01', '...and heres my second post!'),
    ('019f6025-e32f-7256-8c79-0a92e97ea681', 'Just testing my new car.');

INSERT INTO follows (user_id, followed_id)
VALUES
    ('019f6025-bf1d-7a19-a3c4-1c9eb0688f01', '019fb7dd-b1d4-73b2-8a1d-b25bf7fbf207'),
    ('019f6025-e32f-7256-8c79-0a92e97ea681', '019fb7dd-b1d4-73b2-8a1d-b25bf7fbf207');

COMMIT;
