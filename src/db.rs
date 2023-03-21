use log::error;
use sea_orm::{prelude::*, ConnectionTrait, Database, DatabaseBackend};
use std::env;

pub async fn set_up() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(env::var("DATABASE_URL").expect("fatal: no database url")).await?;

    if db.get_database_backend() != DatabaseBackend::Postgres {
        error!("Starship only supports PostgreSQL at this time. Consider using PostgreSQL.");
        panic!("fatal: unsupported sql server, impossible to continue")
    }

    Ok(db)
}
