# rust-base

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-watch](https://crates.io/crates/cargo-watch): `cargo install cargo-watch`
- [sea-orm-cli](https://crates.io/crates/sea-orm-cli): `cargo install sea-orm-cli`

## Run on local
```bash
cargo watch -x run
```

## Database

### Run migrations
```bash
sea-orm-cli migrate run
```

To revert the last migration:
```bash
sea-orm-cli migrate down
```