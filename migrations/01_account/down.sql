-- This file should undo anything in `up.sql`
-- drop trigger set_updated_at on account.contacts;
-- drop trigger set_updated_at on account.users;

drop table if exists account.contacts;
drop table if exists account.users;