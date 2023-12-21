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
                    .add_column(
                        ColumnDef::new(PlanetComponent::Order)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(PlanetComponent::OrderTime)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetComponent::Table)
                    .drop_column(PlanetComponent::Order)
                    .drop_column(PlanetComponent::OrderTime)
                    .to_owned(),
            )
            .await
    }
}
