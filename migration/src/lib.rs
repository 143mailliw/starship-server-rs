pub use sea_orm_migration::prelude::*;

mod m20221115_000001_create_users;
mod m20221120_003244_create_tokens;
mod m20221121_151738_create_planets;
mod m20221121_164753_create_custom_emojis;
mod m20221122_143248_delete_users_following;
mod m20221122_144144_rename_follower_count;
mod m20221122_145255_create_planet_member;
mod m20221202_222651_add_token_verified;
mod m20221203_221004_create_planet_component;
mod m20221206_032207_add_planet_home;
mod m20221210_051909_add_member_created;
mod m20221210_051922_add_component_created;
mod m20221218_002528_create_planet_role;
mod m20230111_135136_rename_role_ids;
mod m20230111_135142_rename_component_ids;
mod m20230111_141433_rename_member_permissions;

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
            Box::new(m20221203_221004_create_planet_component::Migration),
            Box::new(m20221206_032207_add_planet_home::Migration),
            Box::new(m20221210_051909_add_member_created::Migration),
            Box::new(m20221210_051922_add_component_created::Migration),
            Box::new(m20221218_002528_create_planet_role::Migration),
            Box::new(m20230111_135136_rename_role_ids::Migration),
            Box::new(m20230111_135142_rename_component_ids::Migration),
            Box::new(m20230111_141433_rename_member_permissions::Migration),
        ]
    }
}
