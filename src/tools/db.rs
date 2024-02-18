use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::async_trait;
use libsql::params::Params;
use libsql::{Builder, Rows, TransactionBehavior};
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};

use crate::config::app_config::AppConfig;

#[async_trait]
pub trait Database: Send + Sync {
    async fn execute_batch(&self, sql: &str) -> libsql::Result<()>;
    async fn query(&self, sql: &str) -> libsql::Result<Rows>;
    async fn transaction(&self, behavior: TransactionBehavior)
        -> libsql::Result<LibsqlTransaction>;
}

pub struct PooledLibsqlDatabase {
    db: libsql::Database,
    connections: Mutex<Vec<(libsql::Connection, Instant)>>,
    max_idle: Duration,
    semaphore: Arc<Semaphore>,
}

pub struct LibsqlTransaction {
    tx: libsql::Transaction,
}

impl PooledLibsqlDatabase {
    pub async fn new(conf: &AppConfig) -> Self {
        let url = format!("http://{}:{}", conf.db.host, conf.db.port);
        let db = Builder::new_remote(url.clone(), "".to_string())
            .build()
            .await
            .unwrap();

        let connections = Mutex::new(Vec::new());
        let max_idle = conf.db.max_idle;
        let max_connections = conf.db.max_connections;
        let semaphore = Arc::new(Semaphore::new(max_connections));

        Self {
            db,
            connections,
            max_idle,
            semaphore,
        }
    }

    async fn get_connection(&self) -> libsql::Result<(libsql::Connection, OwnedSemaphorePermit)> {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();

        tracing::debug!(
            "Acquired permit, available slots: {}",
            self.semaphore.available_permits()
        );

        let mut connections = self.connections.lock().await;

        connections.retain(|(_, last_used)| last_used.elapsed() < self.max_idle);

        if let Some((existing_conn, _)) = connections.pop() {
            tracing::debug!("Reusing existing connection");
            return Ok((existing_conn, permit));
        }

        tracing::debug!("Creating new connection");
        Ok((self.db.connect()?, permit))
    }

    async fn return_connection(&self, conn: libsql::Connection, _permit: OwnedSemaphorePermit) {
        let mut connections = self.connections.lock().await;
        if connections.len() < self.semaphore.available_permits() {
            connections.push((conn, Instant::now()));
        }
        // permit is implicitly dropped here
    }
}

#[async_trait]
impl Database for PooledLibsqlDatabase {
    async fn execute_batch(&self, sql: &str) -> libsql::Result<()> {
        let (conn, permit) = self.get_connection().await?;
        let result = conn.execute_batch(sql).await;
        self.return_connection(conn, permit).await;

        result
    }

    async fn query(&self, sql: &str) -> libsql::Result<Rows> {
        let (conn, permit) = self.get_connection().await?;
        let result = conn.query(sql, Params::None).await;
        self.return_connection(conn, permit).await;

        result
    }

    async fn transaction(
        &self,
        behavior: TransactionBehavior,
    ) -> libsql::Result<LibsqlTransaction> {
        let (conn, permit) = self.get_connection().await?;
        let tx = conn
            .transaction_with_behavior(behavior)
            .await
            .map(|tx| LibsqlTransaction { tx });
        self.return_connection(conn, permit).await;

        tx
    }
}

impl LibsqlTransaction {
    pub async fn commit(self) -> libsql::Result<()> {
        self.tx.commit().await
    }

    pub async fn rollback(self) -> libsql::Result<()> {
        self.tx.rollback().await
    }
}

#[async_trait]
impl Database for LibsqlTransaction {
    async fn execute_batch(&self, sql: &str) -> libsql::Result<()> {
        self.tx.execute_batch(sql).await
    }

    async fn query(&self, sql: &str) -> libsql::Result<Rows> {
        self.tx.query(sql, Params::None).await
    }

    async fn transaction(
        &self,
        _behavior: TransactionBehavior,
    ) -> libsql::Result<LibsqlTransaction> {
        Err(libsql::Error::Misuse(
            "nested transactions are not supported".to_string(),
        ))
    }
}
