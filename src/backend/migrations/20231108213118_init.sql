-- Create the user table
CREATE TABLE IF NOT EXISTS "user"
(
    id       SERIAL PRIMARY KEY NOT NULL,
    email    text               NOT NULL UNIQUE,
    password text               NOT NULL
);

-- Create the table for storing the systems
CREATE TABLE IF NOT EXISTS system
(
    id        uuid PRIMARY KEY               NOT NULL,
    name      text                           NOT NULL,
    user_id   integer REFERENCES "user" (id) NOT NULL,
    frequency interval                       NOT NULL,
    starts_at timestamp                      NOT NULL,
    deleted   boolean                        NOT NULL DEFAULT FALSE
);

-- Create the table for storing the pings
CREATE TABLE IF NOT EXISTS ping
(
    id        SERIAL PRIMARY KEY          NOT NULL,
    system_id uuid REFERENCES system (id) NOT NULL,
    timestamp timestamp                   NOT NULL DEFAULT NOW()
);

-- SEEDED DATA FOR TESTING PURPOSES
-- Insert "ferris" user.
INSERT INTO "user" (id, email, password)
VALUES (1, 'ferris@example.com',
        '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');
