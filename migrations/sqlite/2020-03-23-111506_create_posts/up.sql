CREATE TABLE posts (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    create_time BIGINT NOT NULL,
    update_time BIGINT NOT NULL,
    comments INTEGER DEFAULT 0
)