-- This file should undo anything in `up.sql`
drop table if exists projects;
drop table if exists templates;
-- drop trigger projects_update_updated_at on projects;