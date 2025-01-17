use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct ReposMigration;

#[async_trait::async_trait]
impl MigrationTrait for ReposMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Branch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Branch::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Branch::Name).string().not_null())
                    .col(
                        ColumnDef::new(Branch::RepositoryUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Branch::Head).string().not_null())
                    .col(
                        ColumnDef::new(Branch::HeadUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Branch::DefaultBranch).boolean().not_null())
                    .col(ColumnDef::new(Branch::StartPoint).boolean().not_null())
                    .col(ColumnDef::new(Branch::From).uuid().null())
                    .col(ColumnDef::new(Branch::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Branch::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Commit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Commit::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Commit::BranchUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Commit::RepositoryUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Commit::Head).string().not_null())
                    .col(ColumnDef::new(Commit::Msg).string().not_null())
                    .col(ColumnDef::new(Commit::Time).big_integer().not_null())
                    .col(ColumnDef::new(Commit::User).string().not_null())
                    .col(ColumnDef::new(Commit::Email).string().not_null())
                    .col(ColumnDef::new(Commit::AvatarUrl).string().null())
                    .col(ColumnDef::new(Commit::Parents).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Repository::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Repository::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Repository::Name).string().not_null())
                    .col(ColumnDef::new(Repository::Description).string().null())
                    .col(
                        ColumnDef::new(Repository::OwnerUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Repository::DefaultBranch)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Repository::Website).string().null())
                    .col(ColumnDef::new(Repository::Visible).boolean().not_null())
                    .col(ColumnDef::new(Repository::Template).boolean().not_null())
                    .col(ColumnDef::new(Repository::Mirrors).boolean().not_null())
                    .col(ColumnDef::new(Repository::Archive).boolean().not_null())
                    .col(ColumnDef::new(Repository::ArchiveTime).big_integer().null())
                    .col(ColumnDef::new(Repository::SshPath).string().not_null())
                    .col(ColumnDef::new(Repository::HttpPath).string().not_null())
                    .col(
                        ColumnDef::new(Repository::StorageNode)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Repository::Fork).boolean().not_null())
                    .col(ColumnDef::new(Repository::ForkUid).uuid().null())
                    .col(ColumnDef::new(Repository::NumsStar).big_integer().not_null())
                    .col(ColumnDef::new(Repository::NumsFork).big_integer().not_null())
                    .col(ColumnDef::new(Repository::NumsWatch).big_integer().not_null())
                    .col(ColumnDef::new(Repository::NumsIssue).big_integer().not_null())
                    .col(ColumnDef::new(Repository::NumsPull).big_integer().not_null())
                    .col(ColumnDef::new(Repository::NumsCommit).big_integer().not_null())
                    .col(ColumnDef::new(Repository::Head).string().not_null())
                    .col(
                        ColumnDef::new(Repository::License)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Repository::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Repository::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tree::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tree::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Tree::CommitUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tree::BranchUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tree::RepositoryUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tree::Hash).string().not_null())
                    .col(
                        ColumnDef::new(Tree::Tree)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Branch::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Commit::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Repository::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tree::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Branch {
    #[sea_orm(iden = "branch")]
    Table,
    Uid,
    Name,
    RepositoryUid,
    Head,
    HeadUid,
    DefaultBranch,
    StartPoint,
    From,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Commit {
    #[sea_orm(iden = "commits")]
    Table,
    Uid,
    BranchUid,
    RepositoryUid,
    Head,
    Msg,
    Time,
    User,
    Email,
    AvatarUrl,
    Parents,
}

#[derive(DeriveIden)]
enum Repository {
    #[sea_orm(iden = "repository")]
    Table,
    Uid,
    Name,
    Description,
    OwnerUid,
    DefaultBranch,
    Website,
    Visible,
    Template,
    Mirrors,
    Archive,
    ArchiveTime,
    SshPath,
    HttpPath,
    StorageNode,
    Fork,
    ForkUid,
    NumsStar,
    NumsFork,
    NumsWatch,
    NumsIssue,
    NumsPull,
    NumsCommit,
    Head,
    License,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tree {
    #[sea_orm(iden = "tree")]
    Table,
    Uid,
    CommitUid,
    BranchUid,
    RepositoryUid,
    Hash,
    Tree,
}
