use std::future::Future;
use std::sync::Arc;

use axum::async_trait;
use libsql::params::Params;
use libsql::{Builder, Connection, Rows};
use tokio::sync::RwLock;

use crate::config::app_config::AppConfig;

#[async_trait]
pub trait Database: Send + Sync {
    async fn execute_batch(&self, sql: &str) -> libsql::Result<()>;
    async fn query(&self, sql: &str) -> libsql::Result<Rows>;
    async fn transaction(&self) -> libsql::Result<LibsqlTransaction>;
}

pub struct LibsqlDatabase {
    db: libsql::Database,
    conn: Arc<RwLock<Connection>>,
}

pub struct LibsqlTransaction {
    tx: libsql::Transaction,
}

impl LibsqlDatabase {
    pub async fn new(conf: &AppConfig) -> Self {
        let url = format!("http://{}:{}", conf.db.host, conf.db.port);
        let db = Builder::new_remote(url.clone(), "".to_string())
            .build()
            .await
            .unwrap();

        let conn = db.connect().unwrap();
        let conn = Arc::new(RwLock::new(conn));

        Self { db, conn }
    }

    async fn reconnect(&self) -> Result<(), libsql::Error> {
        tracing::info!("connection staled, reconnecting...");
        let new_conn = self.db.connect()?;
        let mut conn_write = self.conn.write().await;
        *conn_write = new_conn;

        Ok(())
    }

    async fn with_retry<F, Fut, T>(&self, operation: F) -> libsql::Result<T>
    where
        F: Fn(Arc<RwLock<Connection>>) -> Fut,
        Fut: Future<Output = libsql::Result<T>>,
    {
        let result = operation(self.conn.clone()).await;

        match result {
            Ok(value) => Ok(value),
            Err(libsql::Error::Hrana(_)) => {
                self.reconnect().await?;
                operation(self.conn.clone()).await
            }
            Err(err) => Err(err),
        }
    }
}

#[async_trait]
impl Database for LibsqlDatabase {
    async fn execute_batch(&self, sql: &str) -> libsql::Result<()> {
        self.with_retry(|conn| {
            Box::pin(async move {
                let conn_read = conn.read().await;
                conn_read.execute_batch(sql).await
            })
        })
        .await
    }

    async fn query(&self, sql: &str) -> libsql::Result<Rows> {
        self.with_retry(|conn| {
            Box::pin(async move {
                let conn_read = conn.read().await;
                conn_read.query(sql, Params::None).await
            })
        })
        .await
    }

    async fn transaction(&self) -> libsql::Result<LibsqlTransaction> {
        self.with_retry(|conn| {
            Box::pin(async move {
                let conn_read = conn.read().await;
                conn_read
                    .transaction()
                    .await
                    .map(|tx| LibsqlTransaction { tx })
            })
        })
        .await
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

    async fn transaction(&self) -> libsql::Result<LibsqlTransaction> {
        Err(libsql::Error::Misuse(
            "nested transactions are not supported".to_string(),
        ))
    }
}
