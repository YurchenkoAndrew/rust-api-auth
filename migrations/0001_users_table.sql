CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL UNIQUE,
    email_verified_at TIMESTAMPTZ,
    password VARCHAR(255) NOT NULL,
    remember_token VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT current_timestamp NOT NULL,
    active BOOLEAN NOT NULL
);
