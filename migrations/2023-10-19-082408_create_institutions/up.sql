-- Your SQL goes here

-- Define an ENUM type
create type institution_type as enum (
  'elementary',
  'high',
  'College',
  'university',
  'vocational',
  'technical',
  'other'
);

-- Create institutions table
create table institutions (
  id serial primary key,
  short_name varchar(250) not null unique,
  name varchar(500) not null,
  logo varchar(500),
  contact json,
  in_type institution_type,
  active boolean default true,
  location varchar(500),
  about text,
  established date,
  picture varchar(500),
  created_at timestamp with time zone default current_timestamp
);
