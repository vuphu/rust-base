use dotenv::dotenv;
use std::env;
use std::sync::OnceLock;

static INSTANCE: OnceLock<Env> = OnceLock::new();

pub struct Env {
    pub app_env: String,
    pub app_port: u16,
    pub database_url: String,
}

impl Env {
    pub fn instance() -> &'static Env {
        INSTANCE.get().expect("Config is not initialized")
    }
}

pub fn initialize() {
    dotenv().ok();

    let config = Env {
        app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        app_port: env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3000),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    if INSTANCE.set(config).is_err() {
        eprintln!("Unable to initialize config");
    }
}
