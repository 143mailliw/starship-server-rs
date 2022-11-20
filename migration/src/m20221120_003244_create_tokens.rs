use super::m20221115_000001_create_users::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Token::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Token::User).string().not_null())
                    .col(ColumnDef::new(Token::Ip).string().not_null())
                    .col(ColumnDef::new(Token::Location).string().not_null())
                    .col(ColumnDef::new(Token::Longitude).float())
                    .col(ColumnDef::new(Token::Latitude).float())
                    .col(ColumnDef::new(Token::Browser).string().not_null())
                    .col(ColumnDef::new(Token::OperatingSystem).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-token-user")
                            .from(Token::Table, Token::User)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Token {
    Table,
    Id,
    User,
    Ip,
    Location,
    Latitude,
    Longitude,
    Browser,
    OperatingSystem,
}
