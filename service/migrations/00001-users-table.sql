CREATE TABLE users(
    user_id UUID PRIMARY KEY,
    version UUID NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    updated TIMESTAMP WITH TIME ZONE NOT NULL,

    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    email_address TEXT NOT NULL,
    avatar_url TEXT NULL
);