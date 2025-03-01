use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use std::env;
use std::sync::OnceLock;

pub static DB_POOL: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn setting() -> Result<(), sea_orm::DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connections = Database::connect(db_url.to_string()).await?;
    DB_POOL.set(connections).unwrap();
    Ok(())
}
