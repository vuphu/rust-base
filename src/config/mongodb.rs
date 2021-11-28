use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use once_cell::sync::OnceCell;

use crate::core::common::error::AppError;

static MONGO_DATABASE: OnceCell<Database> = OnceCell::new();

pub fn get_database() -> Option<&'static Database> {
    MONGO_DATABASE.get()
}

pub async fn setup_mongo() -> Result<(), AppError> {
    let mongo_cs = dotenv::var("MONGO_CS")?;
    let mongo_db_name = dotenv::var("MONGO_DB_NAME")?;

    let client_options = ClientOptions::parse(mongo_cs).await?;
    let client = Client::with_options(client_options)?;

    MONGO_DATABASE
        .set(client.database(mongo_db_name.as_str()))
        .unwrap();

    Ok(())
}
