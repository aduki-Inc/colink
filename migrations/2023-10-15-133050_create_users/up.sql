-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(250) NOT NULL UNIQUE,
    password VARCHAR(500) NOT NULL,
    email VARCHAR(250) NOT NULL UNIQUE,
    name VARCHAR(250) NOT NULL,
    active BOOLEAN DEFAULT true,
    bio TEXT,
    dob TIMESTAMP WITH TIME ZONE,
    picture VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT current_timestamp
);
