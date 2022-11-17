use sea_orm::{prelude::*, Database, ConnectionTrait,DatabaseBackend};
use std::env;

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(env::var("DATABASE_URL").expect("fatal: no database url")).await?;
    
    match db.get_database_backend() {
        DatabaseBackend::Postgres => (),
        _ => panic!("fatal: starship-server only supports postgres")
    }

    Ok(db)
}