use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Context, Result};
use api::route::{auth, v1};
use axum::{http::Method, Router};
use registry::AppRegistryImpl;
use shared::config::AppConfig;
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

use shared::env::{which, Environment};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
        .with_target(false);
    // リリースビルドではJSON形式で出力
    #[cfg(not(debug_assertions))]
    let subscriber = subscriber.json();

    // ロガーの登録
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // app_configの初期化
    let app_config = AppConfig::new()?;
    // データベース接続処理
    let pool = connect_database_with(&app_config.database);
    // Redis接続処理
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    // registryの初期化
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config));
    let cors = CorsLayer::new()
        // allow Any headers when accessing the resource
        .allow_headers(cors::Any)
        // allow `GET`,`POST`,`PUT`,`DELETE` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(cors::Any);

    // ルーターの初期化、AppRegistryをRouterに登録
    let app = Router::new()
        .merge(auth::routes())
        .merge(v1::routes())
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
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
