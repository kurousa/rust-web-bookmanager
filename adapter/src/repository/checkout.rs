use crate::database::{
    model::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow},
    ConnectionPool,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::model::checkout::{
    event::{CreateCheckout, UpdateReturned},
    Checkout,
};
use kernel::model::id::{BookId, CheckoutId, UserId};
use kernel::repository::checkout::CheckoutRepository;
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}
impl CheckoutRepositoryImpl {
    /// トランザクション分離レベルをSERIALIZABLEに設定する
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<()> {
        sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut **tx)
            .await
            .map_err(AppError::DatabaseOperationError)?;
        Ok(())
    }
    /// 本のIDから未返却のレコードを取得する
    async fn find_unreturned_by_book_id(&self, book_id: BookId) -> AppResult<Option<Checkout>> {
        let res = sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM
                    checkouts AS c
                    INNER JOIN books AS b USING(book_id)
                WHERE
                    c.book_id = $1
            "#,
            book_id as _,
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?
        .map(Checkout::from);

        Ok(res)
    }
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    /// 貸出操作
    async fn create_checkout(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        self.set_transaction_serializable(&mut tx).await?;

        // 事前チェック
        // 以下のいずれかの条件がNGの場合は処理を中断する
        //
        // - 指定されたIDを持つ蔵書が存在すること
        // - 存在した場合、蔵書が貸出中でないこと
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: CheckoutId",
                        NULL AS "user_id?: UserId"
                    FROM
                        books AS b
                        LEFT OUTER JOIN checkouts AS c
                        USING(book_id)
                    WHERE
                        book_id = $1
                "#,
                event.book_id as _,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::DatabaseOperationError)?;

            match res {
                // 蔵書が存在しない場合
                None => {
                    return Err(AppError::NotFoundError(format!(
                        "指定された書籍({})が見つかりません",
                        event.book_id
                    )))
                }
                // 蔵書が貸出中である場合
                Some(CheckoutStateRow {
                    checkout_id: Some(_),
                    ..
                }) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "指定された書籍({})は貸出中です",
                        event.book_id
                    )))
                }
                // それ以外
                _ => {}
            }
        }

        // 貸出処理の実行
        let checkout_id = CheckoutId::new();
        let res = sqlx::query!(
            r#"
                INSERT INTO checkouts(
                    checkout_id,
                    book_id,
                    user_id,
                    checked_out_at
                )
                VALUES(
                    $1,
                    $2,
                    $3,
                    $4
                )
            "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "貸出処理に失敗しました".into(),
            ));
        }

        tx.commit()
            .await
            .map_err(AppError::DatabaseOperationError)?;

        Ok(())
    }

    /// 返却操作
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        self.set_transaction_serializable(&mut tx).await?;

        // 事前チェック
        // 以下のいずれかの条件がNGの場合は処理を中断する
        //
        // - 指定されたIDを持つ蔵書が存在すること
        // - 存在した場合、蔵書が貸出中でないこと
        {
            let res = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id AS "checkout_id?: CheckoutId",
                        c.user_id AS "user_id?: UserId"
                    FROM
                        books AS b
                        LEFT OUTER JOIN checkouts AS c
                        USING(book_id)
                    WHERE
                        book_id = $1
                "#,
                event.book_id as _,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::DatabaseOperationError)?;

            match res {
                // 蔵書が存在しない場合
                None => {
                    return Err(AppError::NotFoundError(format!(
                        "指定された書籍({})が見つかりません",
                        event.book_id
                    )))
                }
                Some(CheckoutStateRow {
                    checkout_id: Some(c),
                    user_id: Some(u),
                    .. // ignore other fields
                }) if c != event.checkout_id || u != event.returned_by => {
                    return Err(AppError::UnprocessableEntity(format(
                        "指定の貸出(ID({}), ユーザー({}), 書籍({}))は、返却できません",
                        event.checkout_id, event.returned_by, event.book_id
                    )))
                    return Err(AppError::UnprocessableEntity(format!(
                        "指定の貸出(ID({}), ユーザー({}), 書籍({}))は、返却できません",
                        event.checkout_id, event.returned_by, event.book_id
                    )))
                }
                // それ以外
                _ => {}
            }
        }

        // 返却処理の実行
        let res = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts(
                    checkout_id,
                    book_id,
                    user_id,
                    checked_out_at,
                    returned_at
                )
                SELECT checkout_id, book_id, user_id, checked_out_at, $2
                FROM checkouts
                WHERE checkout_id = $1
            "#,
            event.checkout_id as _,
            event.returned_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "返却処理に失敗しました".into(),
            ));
        }

        let res = sqlx::query!(
            r#"
                DELETE FROM checkouts WHERE checkout_id = $1;
            "#,
            event.checkout_id as _,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "返却処理に失敗しました".into(),
            ));
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    /// 全ての未返却の貸出し情報を取得する
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {
        // checkoutsテーブルから全件抽出
        // booksテーブルと内部結合し、蔵書の情報も一緒に取得
        // レコードは、貸出日(checked_out_at)の古い順
        sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM
                    checkouts AS c
                    INNER JOIN books AS b USING(book_id)
                ORDER BY c.checked_out_at ASC
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::DatabaseOperationError)
    }

    // 特定ユーザーの未返却の貸出し情報を取得する
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM
                    checkouts AS c
                    INNER JOIN books AS b USING(book_id)
                WHERE
                    c.user_id = $1
                ORDER BY c.checked_out_at ASC
            "#,
            user_id as _,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::DatabaseOperationError)
    }

    // 特定蔵書の貸出履歴(返却済みも含む)を取得する
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        // 未返却の貸出を取得
        let unreturned_co: Option<Checkout> = self.find_unreturned_by_book_id(book_id).await?;

        // 返却済みの貸出し情報を取得する
        let mut checkout_histories: Vec<Checkout> = sqlx::query_as!(
            ReturnedCheckoutRow,
            r#"
                SELECT
                    rc.checkout_id,
                    rc.book_id,
                    rc.user_id,
                    rc.checked_out_at,
                    rc.returned_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM
                    returned_checkouts AS rc
                    INNER JOIN books AS b USING(book_id)
                WHERE
                    rc.book_id = $1
                ORDER BY rc.returned_at DESC
            "#,
            book_id as _,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?
        .into_iter()
        .map(Checkout::from)
        .collect();

        // 貸出中である場合は、返却済みの履歴の先頭に追加
        if let Some(co) = unreturned_co {
            checkout_histories.insert(0, co);
        }

        Ok(checkout_histories)
    }
}
