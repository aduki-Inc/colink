-- This file should undo anything in `up.sql`
drop table if exists co_link;
drop table if exists roles;
drop table if exists sections;
drop type if exists role_type;
drop trigger co_link_update_updated_at on co_link;
drop trigger sections_update_updated_at on sections;
drop trigger roles_update_updated_at on roles;