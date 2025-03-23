use async_trait::async_trait;
use derive_new::new;
use kernel::model::{
    id::{BookId, UserId},
    {book::event::DeleteBook, list::PaginatedList},
};
use kernel::{
    model::book::{
        event::{CreateBook, UpdateBook},
        Book, BookListOptions,
    },
    repository::book::BookRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::model::book::{BookRow, PaginatedBookRow};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    /// 蔵書レコード作成
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description, user_id)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        Ok(())
    }

    /// 蔵書データ取得
    /// - options: 取得オプション
    ///   - limit: 1ページあたりの取得件数
    ///   - offset: 取得開始位置
    async fn find_all(&self, options: BookListOptions) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;

        let rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT
                    COUNT(*) OVER() AS "total!",
                    b.book_id AS id
                FROM books AS b
                ORDER BY created_at DESC
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        let total = rows.first().map(|r| r.total).unwrap_or_default();
        let book_ids = rows.into_iter().map(|r| r.id).collect::<Vec<BookId>>();

        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id AS book_id,
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS owned_by,
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id IN (SELECT * FROM UNNEST($1::uuid[]))
                ORDER BY b.created_at DESC
            "#,
            &book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        let items = rows.into_iter().map(Book::from).collect();

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })
    }

    /// IDを指定して蔵書データを取得
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id AS book_id,
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS owned_by,
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id = $1
            "#,
            book_id as _ // query_as! マクロによるコンパイル時型チェックを無効化する
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        Ok(row.map(Book::from))
    }

    /// 蔵書データ更新
    async fn update(&self, event: UpdateBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE
                    book_id = $5
                    AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFoundError("specified book not found".into()));
        }
        Ok(())
    }

    /// 蔵書データ削除
    async fn delete(&self, event: DeleteBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE
                    book_id = $1
                    AND user_id = $2
            "#,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        if res.rows_affected() == 0 {
            return Err(AppError::NotFoundError("specified book not found".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::UserRepositoryImpl;
    use kernel::{model::user::event::CreateUser, repository::user::UserRepository};

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        // テスト用にロールおよびユーザーを追加
        sqlx::query!(
            r#"
            INSERT INTO roles(name)
            VALUES ('Admin'), ('User');
        "#
        )
        .execute(&pool)
        .await?;
        let user_repo = UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let user = user_repo
            .create(CreateUser {
                name: "Test User".into(),
                email: "test@example.com".into(),
                password: "test_password".into(),
            })
            .await?;

        let book_repo = BookRepositoryImpl::new(ConnectionPool::new(pool));
        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };
        book_repo.create(book, user.id).await?;

        let options = BookListOptions {
            limit: 20,
            offset: 0,
        };
        let res = book_repo.find_all(options).await?;
        assert_eq!(res.items.len(), 1);

        let book_id = res.items[0].id;
        let res = book_repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner,
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");
        assert_eq!(owner.id, user.id);
        assert_eq!(owner.name, "Test User");

        Ok(())
    }
}
