use shared::error::AppError;

/// Redisのキーを定義するトレイト
/// 関連する型として、RedisValueを意味するValue型を定義する
pub trait RedisKey {
    type Value: RedisValue + TryFrom<String, Error = AppError>;
    fn inner(&self) -> String;
}

/// Redisのバリューを定義するトレイト
pub trait RedisValue {
    fn inner(&self) -> String;
}
