use crate::Env;
use sea_orm::{Database, DatabaseConnection};
use std::sync::{Arc, OnceLock};

pub static DB_POOL: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();

pub async fn initialize() {
    let database_url = Env::instance().database_url.clone();
    let connections = Database::connect(database_url).await.expect("Can't connect to database");
    DB_POOL.set(Arc::new(connections)).unwrap();
}

pub fn get_db_connection() -> Arc<DatabaseConnection> {
    DB_POOL.get().cloned().expect("Database Pool must be initialized before use")
}
