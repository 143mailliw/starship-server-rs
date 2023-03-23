use super::m20221122_145255_create_planet_member::PlanetMember;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetMember::Table)
                    .add_column(
                        ColumnDef::new(PlanetMember::Banned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PlanetMember::Table)
                    .drop_column(PlanetMember::Banned)
                    .to_owned(),
            )
            .await
    }
}
