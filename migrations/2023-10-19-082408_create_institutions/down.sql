-- This file should undo anything in `up.sql`
drop trigger institutions_update_updated_at on institutions;
drop table if exists institutions;
drop type if exists institution_type;