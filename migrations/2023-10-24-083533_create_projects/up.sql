-- Your SQL goes here

-- Create templates table
create table if not exists templates (
  id serial primary key,
  name varchar(500) not null,
  description text not null,
  layout json,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Check if the enum type exists
do $$ 
begin
  if not exists (select 1 from pg_type where typname = 'proposal_type') then
    -- Create the enum type
    create type proposal_type as enum (
      'approval',
      'revised',
      'supplemental',
      'continuation',
      'notice',
      'solicited',
      'other'
    );
  end if;
end $$;

-- Create projects table
create table if not exists projects (
  id serial primary key,
  author integer references users(id) not null,
  template integer references templates(id) not null,
  title varchar(500) not null,
  field varchar(500) not null,
  type proposal_type not null,
  public boolean default true,
  active boolean default true,
  owned boolean not null default false,
  institution integer references institutions(id),
  description text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

create table if not exists proposals(
  id serial primary key,
  project integer unique references projects(id) not null,
  summery text not null
);


-- Create a trigger to run everytime field is updated

create or replace trigger projects_update_updated_at
after update on projects
for each row
execute procedure update_updated_at();

create or replace trigger templates_update_updated_at
after update on templates
for each row
execute procedure update_updated_at();
