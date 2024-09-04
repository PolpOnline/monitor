-- Create users table.
CREATE TABLE IF NOT EXISTS users
(
    id       integer PRIMARY KEY NOT NULL,
    username text                NOT NULL UNIQUE,
    password text                NOT NULL
);

-- Insert "ferris" user.
INSERT INTO users (id, username, password)
VALUES (1, 'ferris',
        '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');
