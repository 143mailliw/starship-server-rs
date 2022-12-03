pub use sea_orm_migration::prelude::*;

mod m20221115_000001_create_users;
mod m20221120_003244_create_tokens;
mod m20221121_151738_create_planets;
mod m20221121_164753_create_custom_emojis;
mod m20221122_143248_delete_users_following;
mod m20221122_144144_rename_follower_count;
mod m20221122_145255_create_planet_member;
mod m20221202_222651_add_token_verified;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221115_000001_create_users::Migration),
            Box::new(m20221120_003244_create_tokens::Migration),
            Box::new(m20221121_151738_create_planets::Migration),
            Box::new(m20221121_164753_create_custom_emojis::Migration),
            Box::new(m20221122_143248_delete_users_following::Migration),
            Box::new(m20221122_144144_rename_follower_count::Migration),
            Box::new(m20221122_145255_create_planet_member::Migration),
            Box::new(m20221202_222651_add_token_verified::Migration),
        ]
    }
}
