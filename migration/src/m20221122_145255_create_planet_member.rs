use super::m20221115_000001_create_users::User;
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
                    .table(PlanetMember::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlanetMember::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlanetMember::Planet).string().not_null())
                    .col(ColumnDef::new(PlanetMember::User).string().not_null())
                    .col(
                        ColumnDef::new(PlanetMember::Roles)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanetMember::Permssions)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member-planet")
                            .from(PlanetMember::Table, PlanetMember::Planet)
                            .to(Planet::Table, Planet::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-member-user")
                            .from(PlanetMember::Table, PlanetMember::User)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanetMember::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PlanetMember {
    Table,
    Id,
    Planet,
    User,
    Roles,
    Permssions,
}
