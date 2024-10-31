CREATE TABLE users (
    username VARCHAR PRIMARY KEY,
    password_hash VARCHAR
);
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    author VARCHAR REFERENCES users(username),
    content VARCHAR,
    likes INT
);
CREATE TABLE likes (
    post UUID REFERENCES posts(id) ON DELETE CASCADE,
    username VARCHAR REFERENCES users(username),
    PRIMARY KEY (post, username)
);