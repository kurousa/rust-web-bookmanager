use crate::helper::{fixture_registry, make_router, v1};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use kernel::repository::health::MockHealthCheckRepository;
use rstest::rstest;
use std::sync::Arc;
use tower::ServiceExt;

#[rstest]
#[tokio::test]
async fn health_check_api_200(fixture_registry: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    let app = make_router(fixture_registry);
    let req = Request::get(&v1("/health")).body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::OK);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn health_check_db_200(mut fixture_registry: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    fixture_registry.expect_health_check_repository().returning(|| {
        let mut mock = MockHealthCheckRepository::new();
        mock.expect_check_db().returning(|| Box::pin(async { true }));
        Arc::new(mock)
    });

    let app = make_router(fixture_registry);
    let req = Request::get(&v1("/health/db")).body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::OK);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn health_check_db_500(mut fixture_registry: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    fixture_registry.expect_health_check_repository().returning(|| {
        let mut mock = MockHealthCheckRepository::new();
        mock.expect_check_db().returning(|| Box::pin(async { false }));
        Arc::new(mock)
    });

    let app = make_router(fixture_registry);
    let req = Request::get(&v1("/health/db")).body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}
