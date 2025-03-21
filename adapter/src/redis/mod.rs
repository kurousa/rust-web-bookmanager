pub mod model;

use redis::{AsyncCommands, Client};
use shared::{config::RedisConfig, error::AppResult};

use self::model::{RedisKey, RedisValue};

pub struct RedisClient {
    client: Client,
}
impl RedisClient {
    /// Redis接続用クライアント初期化
    pub fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }
    /// 期限付きでキーとバリューを保存
    /// 指定した期限(ttl)が経過するとキーとバリューは削除される
    pub async fn set_ex<T: RedisKey>(&self, key: &T, value: &T::Value, ttl: u64) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(key.inner(), value.inner(), ttl).await?;
        Ok(())
    }

    /// キーを指定してバリューを取り出す
    pub async fn get<T: RedisKey>(&self, key: &T) -> AppResult<Option<T::Value>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.get(key.inner()).await?;
        result.map(T::Value::try_from).transpose()
    }

    /// キーを指定して、Redisから該当のキーとバリューを削除する
    pub async fn delete<T: RedisKey>(&self, key: &T) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del(key.inner()).await?;
        Ok(())
    }

    /// Redis接続確認
    pub async fn try_connect(&self) -> AppResult<()> {
        let _ = self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}
