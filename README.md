# crud-rust

Crud API Rust Learning Path

```SHELL
cargo install sqlx-cli --no-default-features --features postgres
cargo install cargo-edit
```

## For migration db

```SHELL
sqlx migrate add -r name_migration
```

## For query compile time run

```SHELL
cargo sqlx prepare
```
