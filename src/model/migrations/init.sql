create table if not exists bun_migrations
(
    id          bigserial
        primary key,
    name        varchar,
    group_id    bigint,
    migrated_at timestamp with time zone default CURRENT_TIMESTAMP not null
);

create table if not exists bun_migration_locks
(
    id         bigserial
        primary key,
    table_name varchar
        unique
);

create table if not exists users
(
    id                 uuid default uuid_generate_v4() not null
        primary key,
    name               varchar                         not null
        unique,
    email              varchar                         not null
        unique,
    encrypted_password varchar                         not null,
    current_sign_in_at timestamp,
    last_sign_in_at    timestamp,
    current_sign_in_ip varchar,
    last_sign_in_ip    varchar,
    created_at         timestamp                       not null,
    updated_at         timestamp                       not null
);

create table if not exists repositories
(
    id                     uuid default uuid_generate_v4() not null
        primary key,
    name                   varchar                         not null,
    owner_id               uuid                            not null,
    head                   varchar                         not null,
    visible                boolean                         not null,
    use_public_storage     boolean                         not null,
    storage_namespace      varchar,
    storage_adapter_params varchar,
    description            varchar,
    creator_id             uuid                            not null,
    created_at             timestamp                       not null,
    updated_at             timestamp                       not null,
    constraint name_owner_unique
        unique (name, owner_id)
);

create table if not exists branches
(
    id            uuid default uuid_generate_v4() not null
        primary key,
    repository_id uuid                            not null,
    commit_hash   bytea                           not null,
    name          varchar                         not null,
    description   varchar,
    creator_id    uuid                            not null,
    created_at    timestamp                       not null,
    updated_at    timestamp                       not null
);

create table if not exists wips
(
    id            uuid default uuid_generate_v4() not null
        primary key,
    current_tree  bytea                           not null,
    base_commit   bytea                           not null,
    repository_id uuid                            not null,
    ref_id        uuid                            not null,
    state         bigint                          not null,
    creator_id    uuid                            not null,
    created_at    timestamp                       not null,
    updated_at    timestamp                       not null,
    constraint creator_id_repository_id_ref_id_unique
        unique (repository_id, ref_id, creator_id)
);

create table if not exists merge_requests
(
    id               uuid default uuid_generate_v4() not null
        primary key,
    mr_sequence      bigint                          not null,
    source_repo_id   uuid                           not null,
    target_repo_id   uuid                           not null,
    target_branch_id uuid                           not null,
    source_branch_id uuid                           not null,
    title            varchar                         not null,
    merge_state      bigint                          not null,
    description      varchar,
    author_id        uuid                           not null,
    created_at       timestamp                       not null,
    updated_at       timestamp                       not null,
    constraint target_seq
        unique (mr_sequence, target_repo_id)
);

create table if not exists commits
(
    hash          bytea     not null,
    repository_id uuid      not null,
    author        jsonb     not null,
    committer     jsonb     not null,
    merge_tag     varchar,
    message       varchar,
    tree_hash     bytea     not null,
    parent_hashes bytea[],
    created_at    timestamp not null,
    updated_at    timestamp not null,
    primary key (hash, repository_id)
);

create table if not exists tags
(
    id            uuid default uuid_generate_v4() not null,
    repository_id uuid                            not null,
    name          varchar,
    tagger        uuid,
    target        bytea,
    message       varchar,
    created_at    timestamp                       not null,
    updated_at    timestamp                       not null,
    primary key (id, repository_id),
    constraint repo_id_name
        unique (repository_id, name)
);

create table if not exists trees
(
    hash          bytea                    not null,
    repository_id uuid                     not null,
    check_sum     bytea,
    type          smallint                 not null,
    size          bigint,
    properties    jsonb                    not null,
    sub_objects   jsonb                    not null,
    created_at    timestamp with time zone not null,
    updated_at    timestamp with time zone not null,
    primary key (hash, repository_id)
);


create table if not exists aksks
(
    id          uuid default uuid_generate_v4() not null
        primary key,
    user_id     uuid                            not null,
    access_key  varchar                         not null
        unique,
    secret_key  varchar                         not null
        unique,
    description varchar,
    created_at  timestamp                       not null,
    updated_at  timestamp                       not null
);


create table if not exists members
(
    id         uuid default uuid_generate_v4() not null
        primary key,
    user_id    uuid                            not null,
    repo_id    uuid                            not null,
    group_id   uuid                            not null,
    created_at timestamp                       not null,
    updated_at timestamp                       not null,
    constraint user_repo_pk
        unique (user_id, repo_id)
);


create table if not exists groups
(
    id         uuid default uuid_generate_v4() not null
        primary key,
    name       varchar                         not null
        unique,
    policies   jsonb                           not null,
    created_at timestamp                       not null,
    updated_at timestamp                       not null
);


create table if not exists policies
(
    id         uuid default uuid_generate_v4() not null
        primary key,
    name       varchar                         not null
        unique,
    statements jsonb                           not null,
    created_at timestamp                       not null,
    updated_at timestamp                       not null
);


create table if not exists usergroup
(
    id         uuid default uuid_generate_v4() not null
        primary key,
    user_id    uuid                            not null,
    group_id   uuid                            not null,
    created_at timestamp                       not null,
    updated_at timestamp                       not null,
    constraint user_group_pk
        unique (user_id, group_id)
);


