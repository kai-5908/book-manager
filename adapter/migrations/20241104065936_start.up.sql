-- Add down migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS books (
    book_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(225) NOT NULL,
    author VARCHAR(225) NOT NULL,
    isbn VARCHAR(225) NOT NULL,
    description VARCHAR(1024) NOT NULL,
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER books_updated_at_trigger
    BEFORE UPDATE ON books FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();