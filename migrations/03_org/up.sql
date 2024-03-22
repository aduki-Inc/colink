-- Your SQL goes here

-- Create schema
create schema if not exists org;

-- Check if the enum type exists
do $$
begin
  if not exists (select 1 from pg_type where typname = 'org_type') then
    -- Create the enum type
    create type org_type as enum (
      'org',
      'ist'
    );
  end if;
end $$;

-- Check if the enum type exists
do $$
begin
  if not exists (select 1 from pg_type where typname = 'institution_type') then
    -- Create the enum type
    create type institution_type as enum (
      'elementary',
      'high',
      'college',
      'university',
      'vocational',
      'technical',
      'org',
      'other'
    );
  end if;
end $$;


-- Create organizations table
create table if not exists org.orgs (
  id serial primary key,
  short_name varchar(250) not null unique,
  name varchar(500) not null,
  logo varchar(500),
  contact jsonb,
  base org_type not null,
  in_type institution_type not null,
  active boolean default false,
  location varchar(500),
  about text,
  established date,
  picture varchar(500),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for orgs
create index idx_org_id on org.orgs(id);
create index idx_org_short_name on org.orgs(short_name);
create index idx_org_base on org.orgs(base);
create index idx_org_active on org.orgs(active);


--Create belongs table
create table if not exists org.belongs (
  id serial primary key,
  active boolean default true,
  author integer references account.users(id) on delete cascade not null,
  org integer references org.orgs(id) on delete cascade not null,
  section integer references platform.sections(id) on delete cascade not null,
  name varchar(500) not null,
  identity varchar(500) not null,
  title varchar(500) not null,
  staff boolean default false,
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Add indices for belongs
create index idx_belong_id on org.belongs(id);
create index idx_belong_active on org.belongs(active);
create index idx_belong_org on org.belongs(org);
create index idx_belong_identity on org.belongs(identity);


-- Create a trigger to run every time field is updated
select diesel_manage_updated_at('org.orgs');
select diesel_manage_updated_at('org.belongs');

