-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        info VARCHAR not null,
        email TEXT NOT NULL,
        password TEXT NOT NULL
);

CREATE TABLE friends (
  id SERIAL PRIMARY KEY,
  id_user1 INTEGER NOT NULL,
  id_user2 INTEGER NOT NULL
);

create or replace function make_friends (
    user1 integer,
    user2 integer) returns void
    as $$
    declare
        user_friends INT;
    begin

    user_friends = (select count(id) from friends where
        (id_user1 = user1 and id_user2 = user2)
        or
        (id_user2 = user1 and id_user1 = user2));

    if (user_friends = 0) then
        insert into friends values (DEFAULT, user1, user2);
    end if;

    return;
    end;
    $$ language plpgsql;


create or replace function delete_friends (
    user1 integer,
    user2 integer) returns void
    as $$
    declare
        user_friends INT;
    begin

    user_friends = (select count(id) from friends where
        (id_user1 = user1 and id_user2 = user2)
        or
        (id_user2 = user1 and id_user1 = user2));

    if (user_friends != 0) then
        delete from friends where id_user1 = user1 and id_user2 = user2;
        delete from friends where id_user1 = user2 and id_user2 = user1;
    end if;

    return;
    end;
    $$ language plpgsql;
                           -- ```
