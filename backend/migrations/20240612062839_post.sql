
CREATE TABLE IF NOT EXISTS "post" (
    id UUID PRIMARY KEY NOT NULL,
    text TEXT NOT NULL,
    author_id UUID NOT NULL,
    CONSTRAINT post_author_id_fk FOREIGN KEY (author_id) REFERENCES "user" (id)
);
