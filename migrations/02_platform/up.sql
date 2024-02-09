-- Your SQL goes here

-- Create schema
create schema if not exists platform;

-- Check if the enum type(role_type) exists, if not create it
do $$
begin
	if not exists (select 1 from pg_type where typname = 'role_type') then
		-- Create the enum type(role_type)
		create type role_type as enum (
			'owner',
			'admin',
			'staff',
			'period'
		);
	end if;
end $$;


-- Check if the enum type(log_type) exists, if not create it
do $$
begin
	if not exists (select 1 from pg_type where typname = 'log_type') then
		-- Create the enum type(log_type)
		create type log_type as enum (
			'request',
			'security',
			'error',
			'action'
		);
	end if;
end $$;

-- Check if the enum type(action_type) exists, if not create it
do $$
begin
	if not exists (select 1 from pg_type where typname = 'action_type') then
		-- Create the enum type(action_type)
		create type action_type as enum (
			'create',
			'read',
			'update',
			'delete'
		);
	end if;
end $$;

-- Create system(co_link) table
create table if not exists platform.co_link (
	id serial primary key,
	name varchar(500) not null,
	description text not null,
	logo varchar(500),
	created_at timestamp with time zone default current_timestamp,
	updated_at timestamp with time zone default current_timestamp
);

-- Create sections table
create table if not exists platform.sections (
	id serial primary key,
	identity varchar(250) unique not null,
	target integer not null,
	name varchar(500) not null,
	description varchar(500),
	created_at timestamp with time zone default current_timestamp,
	updated_at timestamp with time zone default current_timestamp
);

-- Create roles table
create table if not exists platform.roles (
	id serial primary key,
	section integer references platform.sections(id) on delete cascade not null,
	base role_type not null,
	author integer references account.users(id) on delete cascade not null,
	name varchar(500) not null,
	privileges jsonb,
	expiry timestamp with time zone,
	created_at timestamp with time zone default current_timestamp,
	updated_at timestamp with time zone default current_timestamp
);


-- Create approvals table
create table if not exists platform.approvals (
	id serial primary key,
	target integer not null,
	name varchar(250) not null,
	approved boolean default false,
	description text,
	created_at timestamp with time zone default current_timestamp,
	updated_at timestamp with time zone default current_timestamp
);


-- Create logs table
create table if not exists platform.logs (
	id serial primary key,
	audit log_type not null,
	author integer references account.users(id) on delete cascade not null,
	target integer not null,
	name varchar(250) not null,
	action action_type not null,
	verb varchar(500) not null,
	created_at timestamp with time zone default current_timestamp
);


-- Create a trigger to run every time field is updated
select diesel_manage_updated_at('platform.co_link');
select diesel_manage_updated_at('platform.sections');
select diesel_manage_updated_at('platform.roles');
select diesel_manage_updated_at('platform.approvals');