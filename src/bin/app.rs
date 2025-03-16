use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

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
        .with_state(registry);

    // サーバーの起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
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
