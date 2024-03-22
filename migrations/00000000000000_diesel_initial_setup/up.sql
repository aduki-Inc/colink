
-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
-- SELECT diesel_manage_updated_at('users');
-- end

create or replace function diesel_manage_updated_at(_tbl regclass) returns void as $$
begin
    execute format('create trigger set_updated_at before update on %s
    for each row execute procedure diesel_set_updated_at()', _tbl);
end;
$$ language plpgsql;

create or replace function diesel_set_updated_at() returns trigger AS $$
begin
    if (
        new is distinct from old and
        new.updated_at is not distinct from old.updated_at
    ) then
        new.updated_at := current_timestamp;
    end if;
    return new;
end;
$$ language plpgsql;
