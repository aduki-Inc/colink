-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(250) NOT NULL,
    password VARCHAR(50) NOT NULL,
    email VARCHAR(250) NOT NULL,
    name VARCHAR(250) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    bio TEXT,
    dob TIMESTAMP WITH TIME ZONE,
    picture VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT current_timestamp
);
