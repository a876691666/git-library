
## install diesel_cli

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

## create database

```bash
diesel migration run --database-url=./data/users.db
```