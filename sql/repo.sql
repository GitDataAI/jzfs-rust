create table repo
(
    uid             uuid                       not null
        primary key,
    repo_avatar_url text,
    origin          text                       not null,
    visible         boolean   default true     not null,
    use_storage     text      default true     not null,
    bio             text      default ''::text not null,
    branch          uuid[]                     not null,
    forks           integer   default 0        not null,
    stars           integer   default 0        not null,
    create_id       uuid                       not null,
    create_at       timestamp default now()    not null,
    update_at       timestamp default now()    not null,
    fork_from       uuid,
    name            text      default ''::text not null
);

create table brands
(
    uid       uuid                       not null
        primary key,
    repo_id   uuid                       not null,
    name      text      default ''::text not null,
    bio       text      default ''::text not null,
    create_id uuid                       not null,
    create_at timestamp default now()    not null,
    update_at timestamp default now()    not null
);

create table filetree
(
    uid       uuid not null
        primary key,
    repo_id   uuid not null,
    branch_id uuid not null,
    file_tree text not null
);

create table commit
(
    uid       uuid                  not null
        primary key,
    hash      text                  not null,
    author    uuid                  not null,
    message   text default ''::text not null,
    repo_id   uuid                  not null,
    branch_id uuid                  not null,
    file_tree text                  not null
);

create table stars
(
    uid         uuid   not null
        primary key,
    owner_id    uuid   not null,
    stars_repos uuid[] not null
);

