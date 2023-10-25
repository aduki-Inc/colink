-- This file should undo anything in `up.sql`
drop table if exists institutions;
drop type if exists institution_type;
-- drop trigger institutions_update_updated_at on institutions;