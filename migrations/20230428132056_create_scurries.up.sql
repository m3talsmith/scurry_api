-- Add up migration script here
create table if not exists scurries(
                                       uid uuid primary key,
                                       name text,
                                       health int,
                                       age int,
                                       charisma int,
                                       intelligence int,
                                       agility int,
                                       strength int
);
