-- This file should undo anything in `up.sql`
drop trigger projects_update_updated_at on projects;
drop trigger templates_update_updated_at on templates;
drop table if exists projects;
drop table if exists templates;