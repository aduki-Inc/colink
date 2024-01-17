-- Your SQL goes here


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

-- Create templates table
create table if not exists templates (
  id serial primary key,
  name varchar(500) not null,
  description text not null,
  layout jsonb,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);


-- Create projects table
create table if not exists projects (
  id serial primary key,
  author integer references users(id) on delete cascade not null,
  template integer references templates(id) on delete cascade not null,
  title varchar(500) not null,
  field varchar(500) not null,
  type proposal_type not null,
  public boolean not null default true,
  active boolean not null default true,
  owned boolean not null default false,
  institution integer references institutions(id),
  description text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

create table if not exists proposals(
  id serial primary key,
  project integer unique references projects(id) on delete cascade not null,
  summery text not null
);


-- Create a trigger to run everytime field is updated
select diesel_manage_updated_at('projects');
select diesel_manage_updated_at('templates');
