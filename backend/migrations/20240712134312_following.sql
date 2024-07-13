
CREATE TABLE IF NOT EXISTS "following" (
    id UUID NOT NULL PRIMARY KEY,
    from_id UUID NOT NULL,
    to_id UUID NOT NULL,
    CONSTRAINT following_from_id_pk FOREIGN KEY (from_id) REFERENCES "user" (id),
    CONSTRAINT following_to_id_pk FOREIGN KEY (to_id) REFERENCES "user" (id)
);