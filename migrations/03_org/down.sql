-- This file should undo anything in `up.sql`
-- drop trigger set_updated_at on org.belongs;
-- drop trigger set_updated_at on org.orgs;

drop table if exists org.belongs;
drop table if exists org.orgs;
drop type if exists institution_type;
drop type if exists org_type;