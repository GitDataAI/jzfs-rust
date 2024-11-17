create table users
(
    uid        uuid                                   not null
        primary key,
    name       varchar(255),
    username   varchar(255)                           not null,
    password   varchar(255)                           not null,
    avatar_url varchar(255),
    email      varchar(255)                           not null,
    bio        varchar(255),
    links      text[],
    location   varchar(255),
    time_zone  varchar(255),
    language   varchar(255),
    groups     text[],
    create_at  timestamp with time zone default now() not null,
    update_at  timestamp with time zone default now() not null
);


create table pubkey
(
    uid        uuid                                   not null
        primary key,
    user_id    uuid                                   not null,
    name       text                                   not null,
    public_key text                                   not null,
    lastuse_at timestamp with time zone default now() not null,
    create_at  timestamp with time zone default now() not null,
    update_at  timestamp with time zone default now() not null
);

create table pubtoken
(
    uid        uuid                                   not null
        primary key,
    user_id    uuid                                   not null,
    name       text                                   not null,
    token      text                                   not null,
    lastuse_at timestamp with time zone default now() not null,
    create_at  timestamp with time zone default now() not null,
    update_at  timestamp with time zone default now() not null
);


create table groups
(
    uid        uuid                       not null
        primary key,
    name       text                       not null
        unique,
    avatar_url text,
    bio        text      default ''::text not null,
    location   text,
    links      text[]                     not null,
    users      uuid[],
    topics     text[]                     not null,
    pinned     uuid[]                     not null,
    header     uuid                       not null,
    create_to  uuid                       not null,
    create_at  timestamp default now()    not null,
    update_at  timestamp default now()    not null
);