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
                    .table(PlanetComponent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlanetComponent::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlanetComponent::Type).string().not_null())
                    .col(
                        ColumnDef::new(PlanetComponent::ComponentId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlanetComponent::Name).string().not_null())
                    .col(
                        ColumnDef::new(PlanetComponent::PlanetId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-component-planet")
                            .from(PlanetComponent::Table, PlanetComponent::PlanetId)
                            .to(Planet::Table, Planet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanetComponent::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum PlanetComponent {
    Table,
    Id,
    Type,
    ComponentId,
    Name,
    PlanetId,
    Planet,
    Created,
}
