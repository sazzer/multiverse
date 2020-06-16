CREATE TABLE worlds(
    world_id UUID PRIMARY KEY,
    version UUID NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    updated TIMESTAMP WITH TIME ZONE NOT NULL,

    owner_id UUID NOT NULL REFERENCES users (user_id),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    url_slug TEXT NOT NULL
);

ALTER TABLE worlds ADD CONSTRAINT key_worlds_owner_slug UNIQUE (owner_id, url_slug);