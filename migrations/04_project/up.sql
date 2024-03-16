-- Your SQL goes here

-- Create schema project(s)
create schema if not exists project;

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
create table if not exists project.templates (
  id serial primary key,
  author integer references account.users(id) on delete cascade not null,
  name varchar(500) not null,
  description text not null,
  layout jsonb not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create organization templates choice table
create table if not exists project.selections (
  id serial primary key,
  org integer references org.orgs(id) on delete cascade not null,
  template integer references project.templates(id) on delete cascade not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create projects table
create table if not exists project.projects (
  id serial primary key,
  author integer references account.users(id) on delete cascade not null,
  template integer references project.templates(id) on delete cascade not null,
  name varchar(250) not  null unique,
  title varchar(500) not null,
  field varchar(500) not null,
  type proposal_type not null,
  public boolean not null default true,
  active boolean not null default true,
  owned boolean not null default false,
  org integer references org.orgs(id),
  description text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

create table if not exists project.proposals(
  id serial primary key,
  project integer unique references project.projects(id) on delete cascade not null,
  summery text not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create a trigger to run every time field is updated
select diesel_manage_updated_at('project.projects');
select diesel_manage_updated_at('project.templates');
select diesel_manage_updated_at('project.proposals');
select diesel_manage_updated_at('project.selections');
