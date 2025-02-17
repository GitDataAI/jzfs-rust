

create table if not exists users
(
    uid         uuid default uuid_generate_v4() not null
        primary key,
    name        text                            not null,
    username    text                            not null,
    email       text                            not null,
    password    text                            not null,
    description text,
    website     text,
    avatar      text,
    setting     text[]                          not null,
    active      boolean                         not null,
    created_at  timestamp                       not null,
    updated_at  timestamp                       not null,
    timezone    text,
    language    text,
    theme       text,
    location    text
);

create table if not exists repository
(
    uid              uuid default uuid_generate_v4() not null
        primary key,
    name             text                            not null,
    description      text,
    owner_id         uuid                            not null,
    visibility       boolean                         not null,
    fork             uuid,
    default_branch   text                            not null,
    node_uid         uuid                            not null,
    nums_fork        integer                         not null,
    nums_star        integer                         not null,
    nums_watch       integer                         not null,
    nums_issue       integer                         not null,
    nums_pullrequest integer                         not null,
    nums_commit      integer                         not null,
    nums_release     integer                         not null,
    nums_tag         integer                         not null,
    nums_branch      integer                         not null,
    ssh              text                            not null,
    http             text                            not null,
    created_at       timestamp                       not null,
    updated_at       timestamp                       not null,
    created_by       uuid                            not null,
    avatar           text
);

create table if not exists tokens
(
    uid        uuid default uuid_generate_v4() not null
        primary key,
    user_id    uuid                            not null,
    token      text                            not null,
    created_at timestamp                       not null,
    updated_at timestamp                       not null,
    expires_at timestamp                       not null
);

create table if not exists stars
(
    uid           uuid default uuid_generate_v4() not null
        primary key,
    user_id       uuid                            not null,
    repository_id uuid                            not null,
    created_at    timestamp                       not null
);

create table if not exists ssh
(
    uid         uuid default uuid_generate_v4() not null
        primary key,
    user_id     uuid                            not null,
    name        text                            not null,
    description text,
    content     text                            not null,
    created_at  timestamp                       not null,
    updated_at  timestamp                       not null
);

create table if not exists follow
(
    uid        uuid default uuid_generate_v4() not null
        primary key,
    user_id    uuid                            not null,
    target_id  uuid                            not null,
    created_at timestamp                       not null
);

create table if not exists tree
(
    uid      uuid default uuid_generate_v4() not null
        primary key,
    repo_uid uuid                            not null,
    head     text                            not null,
    content  text                            not null,
    branch   text                            not null
);

create index if not exists tree_repo_uid_idx
    on tree (repo_uid);

create index if not exists tree_head_idx
    on tree (head);

