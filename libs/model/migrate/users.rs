use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct UsersMigration;

#[async_trait::async_trait]
impl MigrationTrait for UsersMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Email::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Email::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Email::UserUid).uuid().not_null())
                    .col(ColumnDef::new(Email::Name).string().not_null())
                    .col(ColumnDef::new(Email::Email).string().not_null())
                    .col(
                        ColumnDef::new(Email::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Follow::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Follow::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Follow::UserUid).uuid().not_null())
                    .col(ColumnDef::new(Follow::TargetUid).uuid().not_null())
                    .col(
                        ColumnDef::new(Follow::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SshKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SshKeys::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SshKeys::UserUid).uuid().not_null())
                    .col(ColumnDef::new(SshKeys::Name).string().not_null())
                    .col(ColumnDef::new(SshKeys::Description).string().null())
                    .col(ColumnDef::new(SshKeys::SshKey).string().not_null())
                    .col(ColumnDef::new(SshKeys::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(SshKeys::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Stars::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Stars::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Stars::RepositoryUid).uuid().not_null())
                    .col(ColumnDef::new(Stars::UserUid).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Token::Name).string().not_null())
                    .col(ColumnDef::new(Token::Description).string().null())
                    .col(ColumnDef::new(Token::UserUid).uuid().not_null())
                    .col(ColumnDef::new(Token::Token).string().not_null())
                    .col(ColumnDef::new(Token::Access).integer().not_null())
                    .col(ColumnDef::new(Token::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Token::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::MainEmail).string().not_null())
                    .col(ColumnDef::new(Users::EmailVisible).boolean().not_null())
                    .col(ColumnDef::new(Users::HashPass).string().not_null())
                    .col(ColumnDef::new(Users::Mindset).string().null())
                    .col(ColumnDef::new(Users::State).string().not_null())
                    .col(ColumnDef::new(Users::AvatarUrl).string().null())
                    .col(ColumnDef::new(Users::Company).string().null())
                    .col(ColumnDef::new(Users::JobTitle).string().null())
                    .col(ColumnDef::new(Users::Website).string().null())
                    .col(ColumnDef::new(Users::Social).array(ColumnType::Text).null())
                    .col(ColumnDef::new(Users::Bio).string().null())
                    .col(ColumnDef::new(Users::Location).string().null())
                    .col(ColumnDef::new(Users::Appellative).string().null())
                    .col(ColumnDef::new(Users::Topic).array(ColumnType::Text).null())
                    .col(ColumnDef::new(Users::Pinned).array(ColumnType::Uuid).null())
                    .col(ColumnDef::new(Users::RepositoryLimit).integer().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Users::LastUsed).big_integer().null())
                    .col(ColumnDef::new(Users::Professional).boolean().not_null())
                    .col(ColumnDef::new(Users::ProfessionalEndTime).big_integer().null())
                    .col(ColumnDef::new(Users::ProfessionalStartTime).big_integer().null())
                    .col(ColumnDef::new(Users::Organize).boolean().not_null())
                    .col(ColumnDef::new(Users::Member).array(ColumnType::Uuid).null())
                    .col(ColumnDef::new(Users::Team).array(ColumnType::Uuid).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Watch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Watch::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Watch::UserUid).uuid().not_null())
                    .col(ColumnDef::new(Watch::RepositoryUid).uuid().not_null())
                    .col(ColumnDef::new(Watch::CreatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Email::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Follow::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SshKeys::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Stars::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Watch::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Email {
    #[sea_orm(iden = "emails")]
    Table,
    Uid,
    UserUid,
    Name,
    Email,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Follow {
    #[sea_orm(iden = "follows")]
    Table,
    Uid,
    UserUid,
    TargetUid,
    CreatedAt,
}

#[derive(DeriveIden)]
enum SshKeys {
    #[sea_orm(iden = "ssh_keys")]
    Table,
    Uid,
    UserUid,
    Name,
    Description,
    SshKey,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Stars {
    #[sea_orm(iden = "stars")]
    Table,
    Uid,
    RepositoryUid,
    UserUid,
}

#[derive(DeriveIden)]
enum Token {
    #[sea_orm(iden = "token_keys")]
    Table,
    Uid,
    Name,
    Description,
    UserUid,
    Token,
    Access,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    #[sea_orm(iden = "users")]
    Table,
    Uid,
    Username,
    Name,
    MainEmail,
    EmailVisible,
    HashPass,
    Mindset,
    State,
    AvatarUrl,
    Company,
    JobTitle,
    Website,
    Social,
    Bio,
    Location,
    Appellative,
    Topic,
    Pinned,
    RepositoryLimit,
    CreatedAt,
    UpdatedAt,
    LastUsed,
    Professional,
    ProfessionalEndTime,
    ProfessionalStartTime,
    Organize,
    Member,
    Team,
}

#[derive(DeriveIden)]
enum Watch {
    #[sea_orm(iden = "watch")]
    Table,
    Uid,
    UserUid,
    RepositoryUid,
    CreatedAt,
}