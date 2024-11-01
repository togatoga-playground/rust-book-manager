use model::{RedisKey, RedisValue};
use redis::AsyncCommands;
use shared::{config::RedisConfig, error::AppResult};

pub mod model;

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = redis::Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }
    pub async fn set_ex<T: RedisKey>(&self, key: &T, value: &T::Value, ttl: u64) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set_ex(key.inner(), value.inner(), ttl).await?;
        Ok(())
    }
}
