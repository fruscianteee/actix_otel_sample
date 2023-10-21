-- Add migration script here
CREATE TABLE todo (
    id SERIAL NOT NULL PRIMARY KEY,
    text VARCHAR(255),
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);
