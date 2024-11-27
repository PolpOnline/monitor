use sidekiq::RedisConnectionManager;
use tower_sessions_redis_store::fred::{
    prelude::{ClientLike, RedisConfig as RedisFredConfig, RedisPool as RedisFredPool},
    types::ReconnectPolicy,
};
use tracing::info;

use crate::app::App;

pub type RedisLibPool = bb8::Pool<RedisConnectionManager>;

impl App {
    pub(super) async fn setup_redis_lib() -> color_eyre::Result<RedisLibPool> {
        info!("Redis Lib: Connecting to Redis (to manage workers)...");

        let db_num = 1u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);
        let manager = RedisConnectionManager::new(redis_url)?;
        let redis = bb8::Pool::builder().build(manager).await?;

        info!("Redis Lib: Connected to Redis (to manage workers)");

        Ok(redis)
    }

    pub(super) async fn setup_redis_fred() -> color_eyre::Result<RedisFredPool> {
        info!("Redis Fred: Connecting to Redis (to manage sessions)...");

        let db_num = 0u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);

        let config = RedisFredConfig::from_url(&redis_url)?;

        let pool = RedisFredPool::new(config, None, None, Some(ReconnectPolicy::default()), 6)?;

        pool.init().await?;

        info!("Redis Fred: Connected to Redis (to manage sessions)");

        Ok(pool)
    }
}
