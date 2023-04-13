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
                        ColumnDef::new(PlanetComponent::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(ColumnDef::new(PlanetComponent::ParentId).string())
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk-component-parent")
                            .from(PlanetComponent::Table, PlanetComponent::ParentId)
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
                    .table(PlanetComponent::Table)
                    .drop_column(PlanetComponent::Position)
                    .drop_column(PlanetComponent::ParentId)
                    .to_owned(),
            )
            .await
    }
}
