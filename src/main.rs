use actix_otel_sample::scoped_config;
use actix_web::{web::Data, App, HttpServer};
use opentelemetry::global;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_telemetry() {
    let app_name = "tracing-actix-web-demo";

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint("jaeger:6831")
        .with_service_name(app_name)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer.");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(telemetry)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_ansi(false)
                .with_current_span(false)
                .with_file(true)
                .with_line_number(true)
                .with_span_list(false)
                .with_target(false), // .with_span_events(FmtSpan::CLOSE),
        )
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_telemetry();

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
    .await?;

    // すべてのスパンをエクスポートしてトレーサープロバイダーをシャットダウン
    global::shutdown_tracer_provider();
    Ok(())
}
