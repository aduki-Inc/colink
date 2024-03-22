-- Your SQL goes here

-- Create schema project(s)
create schema if not exists project;

-- Check if the enum type exists
do $$
begin
  if not exists (select 1 from pg_type where typname = 'doc_type') then
    -- Create the enum type
    create type doc_type as enum (
      'doc',
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

-- Add indices for templates
create index idx_template_id on project.templates(id);
create index idx_template_author on project.templates(author);

-- Create organization templates choice table
create table if not exists project.selections (
  id serial primary key,
  org integer references org.orgs(id) on delete cascade not null,
  template integer references project.templates(id) on delete cascade not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for selections(choices)
create index idx_selection_id on project.selections(id);
create index idx_selection_org on project.selections(org);
create index idx_selection_template on project.selections(template);

-- Create projects table
create table if not exists project.projects (
  id serial primary key,
  author integer references account.users(id) on delete cascade not null,
  name varchar(250) not  null,
  title varchar(500) not null,
  field varchar(500) not null,
  public boolean not null default true,
  active boolean not null default true,
  owned boolean not null default false,
  org integer references org.orgs(id),
  description text,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for projects
create index idx_project_id on project.projects(id);
create index idx_project_name on project.projects(name);
create index idx_project_public on project.projects(public);
create index idx_project_active on project.projects(active);
create index idx_project_owned on project.projects(owned);

create table if not exists project.docs(
  id serial primary key,
  name varchar(250) not  null,
  template integer references project.templates(id) on delete cascade not null,
  project integer references project.projects(id) on delete cascade not null,
  kind doc_type not null,
  summery text not null,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for docs
create index idx_doc_id on project.docs(id);
create index idx_doc_template on project.docs(template);
create index idx_doc_project on project.docs(project);
create index idx_doc_kind on project.docs(kind);
create index idx_doc_created on project.docs(created_at);

-- Create a trigger to run every time field is updated
select diesel_manage_updated_at('project.projects');
select diesel_manage_updated_at('project.templates');
select diesel_manage_updated_at('project.docs');
select diesel_manage_updated_at('project.selections');