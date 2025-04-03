use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/health",
        responses (
            (status = 200, description = "health check ok",),
        ),
    )
)]
pub async fn handler_health_check_api() -> StatusCode {
    println!("api health ok!");
    StatusCode::OK
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/health/db",
        responses (
            (status = 200, description = "db health check ok",),
            (status = 500, description = "db health check ng",)
        ),
    )
)]
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
