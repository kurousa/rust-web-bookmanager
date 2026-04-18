use crate::{
    deserialize_json,
    helper::{fixture, make_router, v1, TestRequestExt},
};
use api::model::book::PaginatedBookResponse;
use axum::{body::Body, http::Request};
use kernel::{
    model::{
        book::Book,
        id::{BookId, UserId},
        list::PaginatedList,
        user::BookOwner,
    },
    repository::book::MockBookRepository,
};
use rstest::rstest;
use shared::error::AppError;
use std::sync::Arc;
use tower::ServiceExt;

#[rstest]
#[case("/books", 20, 0)]
#[case("/books?limit=50", 50, 0)]
#[case("/books?limit=50&offset=20", 50, 20)]
#[case("/books?offset=20", 20, 20)]
#[tokio::test]
/// 正常系テスト
async fn show_book_list_with_query_200(
    // fixtureとして、mockオブジェクトを渡す
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    // モックの挙動を設定
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "RustによるWebアプリケーション開発".to_string(),
                isbn: "1234567890".to_string(),
                author: "Yuki Toyoda".to_string(),
                description: "RustによるWebアプリケーション開発".to_string(),
                owner: BookOwner {
                    id: UserId::new(),
                    name: "Yuki Toyoda".to_string(),
                },
                checkout_info: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });
        Arc::new(mock)
    });

    // ルーターを作成
    let app: axum::Router = make_router(fixture);

    // リクエストを作成・送信し、レスポンスのステータスコードを検証
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let resp = app.oneshot(req).await?;
    assert_eq!(resp.status(), axum::http::StatusCode::OK);

    // レスポンスの値を検証
    let result = deserialize_json!(resp, PaginatedBookResponse);
    assert_eq!(result.limit, expected_limit);
    assert_eq!(result.offset, expected_offset);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn delete_book_204(mut fixture: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    let book_id = BookId::new();

    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_delete().returning(|_| Ok(()));
        Arc::new(mock)
    });

    let app = make_router(fixture);

    let req = Request::delete(&v1(&format!("/books/{}", book_id)))
        .bearer()
        .body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), axum::http::StatusCode::NO_CONTENT);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn delete_book_404(mut fixture: registry::MockAppRegistryExt) -> anyhow::Result<()> {
    let book_id = BookId::new();

    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_delete()
            .returning(|_| Err(AppError::NotFoundError("Not Found".into())));
        Arc::new(mock)
    });

    let app = make_router(fixture);

    let req = Request::delete(&v1(&format!("/books/{}", book_id)))
        .bearer()
        .body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), axum::http::StatusCode::NOT_FOUND);

    Ok(())
}

#[rstest]
#[case(r#"{"title": "", "author": "author", "isbn": "isbn", "description": "description"}"#)]
#[case(r#"{"title": "title", "author": "", "isbn": "isbn", "description": "description"}"#)]
#[case(r#"{"title": "title", "author": "author", "isbn": "", "description": "description"}"#)]
#[tokio::test]
/// Error case: Invalid payload for update_book
async fn update_book_400(
    fixture: registry::MockAppRegistryExt,
    #[case] body: &str,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    // Create the router
    let app: axum::Router = make_router(fixture);

    // Create and send the request, then verify the response status code
    let req = Request::put(&v1(&format!("/books/{}", book_id)))
        .header("Content-Type", "application/json")
        .bearer()
        .body(Body::from(body.to_owned()))?;
    let resp = app.oneshot(req).await?;
    assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);

    Ok(())
}

#[rstest]
#[case("/books?limit=-1")]
#[case("/books?offset=aaa")]
#[tokio::test]
/// 異常系テスト
async fn show_book_list_with_query_400(
    // fixtureとして、mockオブジェクトを渡す
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    // モックの挙動を設定
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "rust-web-bookmanager".to_string(),
                isbn: "1234567890".to_string(),
                author: "Ui Kozeki".to_string(),
                description: "rust-web-bookmanager".to_string(),
                owner: BookOwner {
                    id: UserId::new(),
                    name: "Ui Kozeki".to_string(),
                },
                checkout_info: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });
        Arc::new(mock)
    });

    // ルーターを作成
    let app: axum::Router = make_router(fixture);

    // リクエストを作成・送信し、レスポンスのステータスコードを検証
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let resp = app.oneshot(req).await?;
    assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);

    Ok(())
}
