-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL DEFAULT 'Anonymous',
    password VARCHAR(128) NOT NULL,
    role VARCHAR(5) NOT NULL DEFAULT 'ADMIN' CHECK (role IN ('ADMIN', 'USER'))
);