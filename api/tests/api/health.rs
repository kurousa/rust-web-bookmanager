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
#[case(true, StatusCode::OK)]
#[case(false, StatusCode::INTERNAL_SERVER_ERROR)]
#[tokio::test]
async fn health_check_db(
    mut fixture_registry: registry::MockAppRegistryExt,
    #[case] db_status: bool,
    #[case] expected_status: StatusCode,
) -> anyhow::Result<()> {
    fixture_registry
        .expect_health_check_repository()
        .returning(move || {
            let mut mock = MockHealthCheckRepository::new();
            mock.expect_check_db()
                .returning(move || Box::pin(async move { db_status }));
            Arc::new(mock)
        });

    let app = make_router(fixture_registry);
    let req = Request::get(&v1("/health/db")).body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), expected_status);
    Ok(())
}
