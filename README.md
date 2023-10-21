# actix_otel_sample

# 起動

```bash
docker compose watch
```

# db関連

sqlxマクロ用のファイル作成

```bash
cargo sqlx prepare --database-url postgres://postgres:password@localhost/todo
```

sqlx migrate

```bash
sqlx migrate run --database-url postgres://postgres:password@localhost/todo
```

# 技術

- backend
  - actix-web
- template engine
  - askama
- db
  - sqlx
- js framework
  - htmx
- css framework
  - buluma
