use super::m20221121_151738_create_planets::Planet;
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
                    .table(Planet::Table)
                    .add_column(ColumnDef::new(Planet::Home).string())
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk-component-home")
                            .from(Planet::Table, Planet::Home)
                            .to(PlanetComponent::Table, PlanetComponent::Id)
                            .get_foreign_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Planet::Table)
                    .drop_column(Planet::Home)
                    .to_owned(),
            )
            .await
    }
}
