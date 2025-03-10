use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

pub async fn handler_health_check_api() -> StatusCode {
    println!("api health ok!");
    StatusCode::OK
}

pub async fn handler_health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    println!("handler_health_check_db");
    if registry.health_check_repository().check_db().await {
        println!("db health check ok!");
        StatusCode::OK
    } else {
        println!("db health check ng!");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
