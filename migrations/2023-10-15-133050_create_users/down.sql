-- This file should undo anything in `up.sql`
drop trigger users_update_updated_at on users;
drop table if exists users;