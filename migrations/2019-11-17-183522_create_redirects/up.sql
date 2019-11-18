-- Your SQL goes here
CREATE TABLE redirects (
    id TEXT NOT NULL,
    original_url TEXT NOT NULL,
    new_url TEXT NOT NULL,
    PRIMARY KEY (id)
);