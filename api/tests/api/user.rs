use crate::helper::{fixture, fixture_auth, fixture_registry, make_router, v1, TestRequestExt};
use axum::{body::Body, http::Request};
use rstest::rstest;
use tower::ServiceExt;

#[rstest]
#[tokio::test]
async fn register_user_403(fixture: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    let app = make_router(fixture);

    let req = Request::post(&v1("/users"))
        .header("Content-Type", "application/json")
        .bearer()
        .body(Body::from(
            r#"{"name": "test", "email": "test@example.com", "password": "password"}"#,
        ))?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), axum::http::StatusCode::FORBIDDEN);

    Ok(())
}
