ALTER TABLE system
    ADD COLUMN down_after interval NOT NULL DEFAULT '4h'::interval;

ALTER TABLE system
    ADD COLUMN down_sent_email BOOLEAN NOT NULL DEFAULT FALSE;