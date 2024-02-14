use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use etcd_client::{Client, LockOptions, LockResponse};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tonic::Code::AlreadyExists;

use crate::config::app_config::AppConfig;
use crate::tools::error::CustomError;

pub struct Locker {
    client: Arc<Mutex<Client>>,
    default_ttl: Duration,
    backoff: Duration,
}

impl Locker {
    pub async fn new(conf: &AppConfig) -> Self {
        let addr = format!("http://{}:{}", conf.etcd.host, conf.etcd.port);
        let client = Client::connect([addr], None).await.unwrap();
        let default_ttl = conf.etcd.ttl;
        let backoff = conf.etcd.backoff;

        Self {
            client: Arc::new(Mutex::new(client)),
            default_ttl,
            backoff,
        }
    }

    pub async fn with_lock<F, Fut, T>(&self, key: String, f: F) -> Result<T, CustomError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, CustomError>>,
    {
        let retry_until = Instant::now() + self.default_ttl;

        loop {
            match self.try_lock(&key).await {
                Ok(lock) => {
                    // successfully obtained distributed lock
                    let result = f().await;

                    if self.unlock(lock).await.is_err() {
                        tracing::warn!("Failed to unlock etcd lock for key {}", key);
                    }

                    return result;
                }

                Err(etcd_client::Error::GRpcStatus(ref status))
                    if status.code() == AlreadyExists =>
                {
                    // lock is being held
                    if Instant::now() >= retry_until {
                        // retries exhausted
                        return Err(CustomError::LockHeld(format!(
                            "Lock is being held for key {}",
                            key
                        )));
                    }

                    sleep(self.backoff).await;
                }

                Err(err) => {
                    // unexpected error like IO error, transport error, etc.
                    return Err(CustomError::Unexpected(anyhow::Error::new(err)));
                }
            }
        }
    }

    async fn try_lock(&self, key: &str) -> Result<LockResponse, etcd_client::Error> {
        let mut client = self.client.lock().await;
        let lease = client
            .lease_grant(self.default_ttl.as_secs() as i64, None)
            .await?;

        let lock_options = LockOptions::new().with_lease(lease.id());
        let lock = client.lock(key, Some(lock_options)).await?;

        Ok(lock)
    }

    async fn unlock(&self, lock: LockResponse) -> Result<(), etcd_client::Error> {
        let mut client = self.client.lock().await;
        client.unlock(lock.key()).await?;

        Ok(())
    }
}
