-- Add migration script here
CREATE TYPE visibility AS ENUM ('public', 'private');

ALTER TABLE system ADD column visibility visibility DEFAULT 'public' NOT NULL;
