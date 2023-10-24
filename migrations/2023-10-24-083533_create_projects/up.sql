-- Your SQL goes here

-- Create templates table
create table templates (
  id serial primary key,
  name varchar(500) not null,
  description text not null,
  layout json
);

-- Create projects table
create table projects (
  id serial primary key,
  author integer references users(id) not null,
  template integer references templates(id) not null,
  title varchar(500) not null,
  field varchar(500) not null,
  public boolean default true,
  active boolean default true,
  summery text,
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
create trigger projects_update_updated_at
after update on projects
for each row
execute procedure update_updated_at();