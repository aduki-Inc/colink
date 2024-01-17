-- Your SQL goes here

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
      'other'
    );
  end if;
end $$;


-- Create institutions table
create table if not exists institutions (
  id serial primary key,
  short_name varchar(250) not null unique,
  name varchar(500) not null,
  logo varchar(500),
  contact jsonb,
  in_type institution_type not null,
  active boolean default true,
  location varchar(500),
  about text,
  established date,
  picture varchar(500),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);


--Create belongs table 
create table if not exists belongs (
  id serial primary key,
  author integer references users(id) on delete cascade not null,
  institution integer references institution(id) on delete cascade not null;
  name varchar(500) not null;
  identity varchar(500) not null,
  title varchar(500) not null;
  staff boolean default false;
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);


-- Create a trigger to run everytime field is updated
select diesel_manage_updated_at('institutions');
select diesel_manage_updated_at('belongs');

