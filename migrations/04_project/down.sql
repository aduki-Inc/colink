-- This file should undo anything in `up.sql`

-- drop trigger set_updated_at on project.selections;
-- drop trigger set_updated_at on project.proposals;
-- drop trigger set_updated_at on project.templates;
-- drop trigger set_updated_at on project.projects;

drop table if exists project.selections;
drop table if exists project.proposals;
drop table if exists project.templates;
drop table if exists project.projects;

drop type if exists doc_type;