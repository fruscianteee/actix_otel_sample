use actix_otel_sample::scoped_config;
use actix_web::{web::Data, App, HttpServer};
use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URLが設定されていません");
    let postgres = Data::new(
        sqlx::PgPool::connect(&database_url)
            .await
            .expect("データベースに接続できませんでした"),
    );
    info!("start server.");
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(scoped_config)
            .app_data(postgres.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
