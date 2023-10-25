use super::m20221218_002528_create_planet_role::PlanetRole;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetRole::Table)
                    .rename_column(PlanetRole::PlanetId, PlanetRole::Planet)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetRole::Table)
                    .rename_column(PlanetRole::Planet, PlanetRole::PlanetId)
                    .to_owned(),
            )
            .await
    }
}
