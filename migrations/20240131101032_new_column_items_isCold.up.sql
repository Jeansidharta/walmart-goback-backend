-- Add up migration script here
ALTER TABLE Item ADD COLUMN is_cold INTEGER default FALSE NOT NULL;
