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

-- Add indices for users
create index idx_user_id on account.users(id);
create index idx_username on account.users(username);
create index idx_active on account.users(active);

-- Create table for (Follow system)
create table if not exists account.contacts (
  id serial primary key,
  user_from integer references account.users(id) on delete cascade not null,
  user_to integer references account.users(id) on delete cascade not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for contacts
create index idx_contact_id on account.contacts(id);
create index idx_user_from on account.contacts(user_from);
create index idx_user_to on account.contacts(user_from);

-- Create a function to update updated_at column
select diesel_manage_updated_at('account.users');
select diesel_manage_updated_at('account.contacts');