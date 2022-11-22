pub use sea_orm_migration::prelude::*;

mod m20221115_000001_create_users;
mod m20221120_003244_create_tokens;
mod m20221121_151738_create_planets;
mod m20221121_164753_create_custom_emojis;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221115_000001_create_users::Migration),
            Box::new(m20221120_003244_create_tokens::Migration),
            Box::new(m20221121_151738_create_planets::Migration),
            Box::new(m20221121_164753_create_custom_emojis::Migration),
        ]
    }
}
