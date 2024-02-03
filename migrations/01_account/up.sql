-- Your SQL goes here


-- Create schema
create schema if not exists account;

-- Create table for (Users)
create table if not exists account.users (
  id serial primary key,
  username varchar(250) not  null unique,
  password varchar(500) not null,
  email varchar(250) not null unique,
  name varchar(250) not null,
  active boolean default true,
  bio text,
  dob timestamp with time zone,
  picture varchar(500),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create a function to update updated_at column
select diesel_manage_updated_at('account.users');