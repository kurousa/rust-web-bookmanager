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
        conn.set_ex::<_, _, ()>(key.inner(), value.inner(), ttl)
            .await?;        
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
        conn.del::<_, ()>(key.inner()).await?;
        Ok(())
    }

    /// Redis接続確認
    pub async fn try_connect(&self) -> AppResult<()> {
        let _ = self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::error::AppError;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TestContent {
        pub name: String,
    }
    impl TryFrom<String> for TestContent {
        type Error = AppError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            Ok(Self { name: value })
        }
    }
    impl RedisValue for TestContent {
        fn inner(&self) -> String {
            self.name.to_string()
        }
    }

    pub struct TestContentKey(String);
    impl RedisKey for TestContentKey {
        type Value = TestContent;
        fn inner(&self) -> String {
            self.0.to_string()
        }
    }

    #[sqlx::test]
    async fn test_con() -> anyhow::Result<()> {
        let config = RedisConfig {
            host: std::env::var("REDIS_HOST").unwrap(),
            port: std::env::var("REDIS_PORT").unwrap().parse().unwrap(),
        };
        let client = RedisClient::new(&config)?;

        // 存在しないトークンを取得
        let res_no_exist_token = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_no_exist_token.is_none());

        // トークンをセット
        client
            .set_ex(
                &TestContentKey("redis:key".to_string()),
                &TestContent {
                    name: "test".to_string(),
                },
                1000,
            )
            .await?;

        // トークンが取得できる
        let res_exist_token = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert_eq!(
            res_exist_token,
            Some(TestContent {
                name: "test".to_string()
            })
        );

        // トークンを削除
        client
            .delete(&TestContentKey("redis:key".to_string()))
            .await?;

        // トークンを取得できない
        let res_deleted = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_deleted.is_none());

        Ok(())
    }
}
