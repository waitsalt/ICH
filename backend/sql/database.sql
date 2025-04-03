create table "user" (
    "user_id" bigserial primary key,
    "user_name" text not null unique,
    "user_desc" text not null default '',
    "user_password" text not null default '',
    "user_email" text not null unique default '',
    "user_phone" text not null default '',
    "user_avatar_url" text not null default '',
    "user_level" int2 not null default 0,
    "user_status" int2 not null default 0,
    "user_create_time" timestamp
    with
        time zone not null default (NOW () AT TIME ZONE 'UTC'),
        "user_update_time" timestamp
    with
        time zone not null default (NOW () AT TIME ZONE 'UTC'),
        "user_identity" int2 not null default 0
);

CREATE TABLE ich (
    ich_id BIGSERIAL PRIMARY KEY,
    ich_code VARCHAR(255) NOT NULL,
    ich_name VARCHAR(255) NOT NULL,
    ich_publish VARCHAR(100) NOT NULL,
    ich_category VARCHAR(100) NOT NULL,
    ich_location VARCHAR(100) NOT NULL,
    ich_apply_location VARCHAR(100) NOT NULL,
    ich_protect_unit VARCHAR(255) NOT NULL,
    ich_content TEXT NOT NULL,
    ich_create TIMESTAMPTZ NOT NULL DEFAULT (NOW () AT TIME ZONE 'UTC'),
    ich_uploader BIGINT NOT NULL
);

CREATE TABLE posts (
    post_id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    ich_id BIGINT NOT NULL,
    post_title TEXT NOT NULL,
    post_content TEXT NOT NULL,
    post_media_urls JSONB,
    post_type SMALLINT NOT NULL,
    post_status SMALLINT NOT NULL,
    post_likes INTEGER NOT NULL DEFAULT 0,
    post_favorites INTEGER NOT NULL DEFAULT 0,
    post_create_time TIMESTAMPTZ NOT null default (NOW () AT TIME ZONE 'UTC'),
    post_update_time TIMESTAMPTZ NOT null default (NOW () AT TIME ZONE 'UTC')
    -- Foreign key constraints
    --    FOREIGN KEY (user_id) REFERENCES user(user_id),
    --    FOREIGN KEY (ich_id) REFERENCES ich(ich_id)
);
