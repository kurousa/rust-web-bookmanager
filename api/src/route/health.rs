use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::health::{handler_health_check_api, handler_health_check_db};

pub fn build_health_check_routers() -> Router<AppRegistry> {
    let health_check_routers = Router::new()
        .route("/", get(handler_health_check_api))
        .route("/db", get(handler_health_check_db));
    Router::new().nest("/health", health_check_routers)
}
