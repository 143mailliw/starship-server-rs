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
                    .table(Planet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Planet::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Planet::Name).string().not_null())
                    .col(ColumnDef::new(Planet::Created).timestamp().not_null())
                    .col(ColumnDef::new(Planet::Owner).string().not_null())
                    .col(
                        ColumnDef::new(Planet::Private)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Planet::FollowerCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Planet::Featured)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Planet::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Planet::Partnered)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Planet::FeaturedDescription)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Planet::Banned)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Planet::Css).string().not_null().default(""))
                    .col(ColumnDef::new(Planet::Description).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-planet-owner")
                            .from(Planet::Table, Planet::Owner)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Planet::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Planet {
    Table,
    Id,
    Created,
    Name,
    Owner,
    Private,
    FollowerCount, // renamed to member_count
    MemberCount,

    Featured,
    Verified,
    Partnered,
    FeaturedDescription,

    Banned,

    Css,
    Description,
}
