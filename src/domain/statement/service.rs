use std::sync::Arc;

use anyhow::Context;
use derive_new::new;
use libsql::Connection;
use time::OffsetDateTime;

use crate::domain::client::service::ClientService;
use crate::domain::statement::model::{Statement, StatementBalance, StatementTransaction};
use crate::domain::transaction::model::Transaction;
use crate::domain::transaction::service::TransactionService;
use crate::tools::error::CustomError;

#[derive(new)]
pub struct StatementService {
    client_service: Arc<ClientService>,
    transaction_service: Arc<TransactionService>,
    db: Arc<Connection>,
}

impl StatementService {
    pub async fn find(&self, client_id: u32) -> Result<Statement, CustomError> {
        let _client = self.client_service.find(client_id).await?;

        let tx = self
            .db
            .transaction()
            .await
            .context("failed to start a transaction")?;

        let meta = self.client_service.find_meta(client_id, Some(&tx)).await?;
        let transactions = self
            .transaction_service
            .find_latest(client_id, Some(&tx))
            .await?;

        let balance =
            StatementBalance::new(meta.balance, meta.negative_limit, OffsetDateTime::now_utc());
        let statement_transactions = transactions
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<_>>();

        tx.commit().await.context("failed to commit transaction")?;

        Ok(Statement::new(balance, statement_transactions))
    }
}

impl Into<StatementTransaction> for Transaction {
    fn into(self) -> StatementTransaction {
        StatementTransaction {
            amount: self.amount,
            operation: self.operation,
            description: self.description,
            created_at: self.created_at,
        }
    }
}
