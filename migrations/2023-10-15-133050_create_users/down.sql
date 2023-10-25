-- This file should undo anything in `up.sql`
drop table if exists users;
drop trigger users_update_updated_at on users;