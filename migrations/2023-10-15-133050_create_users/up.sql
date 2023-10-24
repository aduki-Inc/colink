-- Your SQL goes here
create table if not exists users (
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

-- Create a function to update updated_at column
create or replace function update_updated_at()
returns trigger as $$
begin
  new.updated_at = now();
  return new;
end;
$$ language plpgsql;


-- Create a trigger to run everytime field is updated
create trigger users_update_updated_at
after update on users
for each row
execute procedure update_updated_at();

