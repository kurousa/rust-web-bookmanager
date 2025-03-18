use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Context, Error, Result};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

use shared::env::{which, Environment};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// ロガーの初期化
fn init_logger() -> Result<()> {
    // ログレベルの設定
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログのフォーマット設定
    // ファイル名、行番号、ターゲット名を表示
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    // ロガーの登録
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // app_conigの初期化
    let app_config = AppConfig::new()?;
    // データベース接続処理
    let pool = connect_database_with(&app_config.database);
    // registryの初期化
    let registry = AppRegistry::new(pool);

    // ルーターの初期化、AppRegistryをRouterに登録
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        // TODO: implement
        //.merge(vi::routes())
        //.merge(auth::routes())
        .layer(cors())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .latency_unit(LatencyUnit::Millis),
        )
        .with_state(registry);

    // サーバーの起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on: {}", addr);
    axum::serve(listener, app)
        .await
        // 起動失敗時のログを tracing::error! で出力する
        .context("Unexpected error in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error",
            )
        })
}

// #[tokio::test]
// async fn test_handler_health() {
//     let status_code = handler_health().await;
//     assert_eq!(status_code, StatusCode::OK);
// }

// #[sqlx::test]
// async fn test_handler_health_check_db(pool: sqlx::PgPool) {
//     let status_code = handler_health_check_db(State(pool)).await;
//     assert_eq!(status_code, StatusCode::OK);
// }
