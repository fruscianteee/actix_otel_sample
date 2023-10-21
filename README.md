# actix_otel_sample

# 起動
```
docker compose watch
```

# db関連

sqlxマクロ用のファイル作成
```
cargo sqlx prepare --database-url postgres://postgres:password@localhost/todo
```

sqlx migrate
```
sqlx migrate run --database-url postgres://postgres:password@localhost/todo
```
