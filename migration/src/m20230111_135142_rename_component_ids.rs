use super::m20221203_221004_create_planet_component::PlanetComponent;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetComponent::Table)
                    .rename_column(PlanetComponent::PlanetId, PlanetComponent::Planet)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetComponent::Table)
                    .rename_column(PlanetComponent::Planet, PlanetComponent::PlanetId)
                    .to_owned(),
            )
            .await
    }
}
