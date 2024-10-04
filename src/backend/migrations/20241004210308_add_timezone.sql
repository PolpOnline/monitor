-- Add migration script here
ALTER TABLE "user" ADD COLUMN timezone text NOT NULL DEFAULT 'UTC';