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
  contact json,
  in_type institution_type not null,
  active boolean default true,
  location varchar(500),
  about text,
  established date,
  picture varchar(500),
  created_at timestamp with time zone default current_timestamp,
  updated_at timestamp with time zone default current_timestamp
);

-- Create a function to update updated_at column
create function update_updated_at()
returns trigger as $$
begin
  new.updated_at = now();
  return new;
end;
$$ language plpgsql;

-- Create a trigger to run everytime field is updated
create trigger institutions_update_updated_at
after update on institutions
for each row
execute procedure update_updated_at();