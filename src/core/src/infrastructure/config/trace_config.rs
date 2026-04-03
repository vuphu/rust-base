use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn initialize() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new("info").add_directive("sqlx=off".parse().unwrap())
            }),
        )
        .with(
            fmt::layer()
                .with_target(false)
                .with_file(false)
                .with_line_number(false)
                .with_filter(filter_fn(|meta| meta.level() == &Level::INFO)),
        )
        .with(
            fmt::layer()
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .with_filter(LevelFilter::WARN),
        )
        .init();
}
