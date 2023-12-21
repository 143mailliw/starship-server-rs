use super::m20221121_151738_create_planets::Planet;
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
                    .drop_column(Planet::Banned)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Planet::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Planet::Banned)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
