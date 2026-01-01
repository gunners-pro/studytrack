-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create a new table 'users' with a primary key and columns
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);