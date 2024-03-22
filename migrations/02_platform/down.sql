-- This file should undo anything in `up.sql`
-- drop trigger set_updated_at on platform.roles;
-- drop trigger set_updated_at on platform.sections;
-- drop trigger set_updated_at on platform.co_link;
-- drop trigger set_updated_at on platform.logs;

drop table if exists platform.roles;
drop table if exists platform.sections;
drop table if exists platform.co_link;
drop table if exists platform.logs;

drop type if exists role_type;
drop type if exists log_type;
drop type if exists action_type;
drop type if exists section_type;

