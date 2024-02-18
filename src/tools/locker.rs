use std::future::Future;
use std::time::{Duration, Instant};

use redis::RedisError;
use redlock::{RedLock, RedLockGuard};

use crate::config::app_config::AppConfig;
use crate::tools::error::CustomError;
use crate::tools::metrics::OPS_HISTOGRAM;

pub struct Locker {
    client: RedLock,
    default_ttl: Duration,
}

impl Locker {
    pub async fn new(conf: &AppConfig) -> Self {
        let addr = format!("redis://{}:{}/", conf.redis.host, conf.redis.port);
        let default_ttl = conf.redis.ttl;

        let client = RedLock::new(vec![addr]);

        Self {
            client,
            default_ttl,
        }
    }

    pub async fn with_lock<F, Fut, T>(&self, key: String, f: F) -> Result<T, CustomError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, CustomError>>,
    {
        let start = Instant::now();

        match self.try_lock(&key).await {
            Ok(_lock) => {
                // successfully obtained distributed lock
                let acquired_time = start.elapsed().as_secs_f64();
                OPS_HISTOGRAM
                    .with_label_values(&["acquire_lock"])
                    .observe(acquired_time);

                f().await
                // lock is implicitly dropped here (see Drop trait implementation for RedLockGuard)
            }

            Err(err) => {
                // unexpected error like IO error, transport error, etc.
                Err(CustomError::Unexpected(anyhow::Error::new(err)))
            }
        }
    }

    async fn try_lock(&self, key: &str) -> Result<RedLockGuard, RedisError> {
        let lock = self
            .client
            .acquire_async(key.as_bytes(), self.default_ttl.as_millis() as usize)
            .await?;

        Ok(lock)
    }
}
