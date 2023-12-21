use super::m20221115_000001_create_users::User;
use super::m20221121_151738_create_planets::Planet;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CustomEmoji::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomEmoji::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CustomEmoji::Owner).string().not_null())
                    .col(ColumnDef::new(CustomEmoji::Planet).string())
                    .col(ColumnDef::new(CustomEmoji::Name).string().not_null())
                    .col(ColumnDef::new(CustomEmoji::Url).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-emoji-owner")
                            .from(CustomEmoji::Table, CustomEmoji::Owner)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-emoji-planet")
                            .from(CustomEmoji::Table, CustomEmoji::Planet)
                            .to(Planet::Table, Planet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomEmoji::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CustomEmoji {
    Table,
    Id,
    Owner,
    Planet,
    Name,
    Url,
}
