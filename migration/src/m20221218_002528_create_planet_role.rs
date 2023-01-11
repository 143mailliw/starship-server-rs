use sea_orm_migration::prelude::*;

use crate::m20221121_151738_create_planets::Planet;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlanetRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlanetRole::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PlanetRole::Name)
                            .string()
                            .not_null()
                            .default("New Role"),
                    )
                    .col(ColumnDef::new(PlanetRole::Color).string_len(7).not_null())
                    .col(
                        ColumnDef::new(PlanetRole::Permissions)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlanetRole::PlanetId).string().not_null())
                    .col(
                        ColumnDef::new(PlanetRole::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(PlanetRole::Default)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-planet")
                            .from(PlanetRole::Table, PlanetRole::PlanetId)
                            .to(Planet::Table, Planet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanetRole::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PlanetRole {
    Table,
    Id,
    Name,
    Color,
    Permissions,
    PlanetId,
    Position,
    Default,
}
