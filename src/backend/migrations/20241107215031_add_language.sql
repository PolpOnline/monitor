-- Add migration script here
ALTER TABLE "user"
    ADD COLUMN language VARCHAR(8) NOT NULL DEFAULT 'en';