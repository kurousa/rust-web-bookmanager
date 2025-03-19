use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    NotFoundError(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report),
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベースエラーが発生しました。")]
    DatabaseOperationError(#[source] sqlx::Error),
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KVSError(#[from] redis::RedisError),
    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("ログインに失敗しました。")]
    UnauthenticatedError,
    #[error("認可に失敗しました。")]
    UnauthorizedError,
    #[error("許可されていない操作です。")]
    ForbiddenError,
    #[error("{0}")]
    ConversionEntityError(String),
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) | AppError::ConvertToUuidError(_) => {
                StatusCode::BAD_REQUEST
            }
            AppError::UnauthenticatedError | AppError::ForbiddenError => StatusCode::FORBIDDEN,
            AppError::UnauthorizedError => StatusCode::UNAUTHORIZED,
            e @ (AppError::TransactionError(_)
            | AppError::DatabaseOperationError(_)
            | AppError::NoRowsAffectedError(_)
            | AppError::KVSError(_)
            | AppError::BcryptError(_)
            | AppError::ConversionEntityError(_)
            | AppError::InternalError(_)) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error occurred"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}

/// エラーとしてAppError型を扱い可能なResult型
pub type AppResult<T> = Result<T, AppError>;
