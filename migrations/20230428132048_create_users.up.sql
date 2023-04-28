-- Add up migration script here
CREATE TABLE if not exists users(
    uid uuid primary key,
    name text,
    password text,
    created_at timestamp default now()
);