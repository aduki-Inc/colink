-- Your SQL goes here

-- Check if the enum type(role_type) exists, if not create it
do $$ 
begin
  if not exists (select 1 from pg_type where typname = 'role_type') then
    -- Create the enum type(role_type)
    create type role_type as enum (
      'dev',
      'super',
      'user'
    );
  end if;
end $$;

-- Create system(co_link) table
create table if not exists co_link (
  id serial primary key,
  name varchar(500) not null,
  description text not null,
  logo varchar(500),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create sections table
create table if not exists sections (
  id serial primary key,
  name varchar(500) unique not null,
  target_id integer not null,
  target_name varchar(500) not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create roles table
create table if not exists roles (
  id serial primary key,
  section integer references sections(id),
  type role_type not null,
  author integer references users(id) not null,
  name varchar(500),
  privileges json,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create a trigger to run everytime field is updated
create or replace trigger co_link_update_updated_at
after update on co_link
for each row
execute procedure update_updated_at();

create or replace trigger sections_update_updated_at
after update on sections
for each row
execute procedure update_updated_at();

create or replace trigger roles_update_updated_at
after update on roles
for each row
execute procedure update_updated_at();
