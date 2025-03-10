use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

// sharedに移動した
// struct DatabaseConfig {
//     pub host: String,
//     pub port: u16,
//     pub username: String,
//     pub password: String,
//     pub database: String,
// }
// impl From<DatabaseConfig> for PgConnectOptions {
//     fn from(cfg: DatabaseConfig) -> Self {
//         Self::new()
//             .host(&cfg.host)
//             .port(cfg.port)
//             .username(&cfg.username)
//             .password(&cfg.password)
//             .database(&cfg.database)
//     }
// }

// adapterに移動した
// fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
//     PgPool::connect_lazy_with(cfg.into())
// }

// apiに移動した
// async fn handler_health() -> StatusCode {
//     StatusCode::OK
// }

// async fn handler_health_check_db(State(db): State<PgPool>) -> StatusCode {
//     let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
//     match connection_result {
//         Ok(_) => StatusCode::OK,
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: app_conigの初期化

    // TODO: adapterの初期化

    // TODO: registryからの取得を行う

    let app = Router::new();
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
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
