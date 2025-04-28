-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS urls (
    id SERIAL PRIMARY KEY, 
    long_url TEXT NOT NULL,
    short_url TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS  test_urls (
    id SERIAL PRIMARY KEY, 
    long_url TEXT NOT NULL,
    short_url TEXT NOT NULL
);