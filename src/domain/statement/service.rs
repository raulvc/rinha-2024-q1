use std::sync::Arc;

use anyhow::Context;
use derive_new::new;
use libsql::TransactionBehavior::ReadOnly;
use time::OffsetDateTime;

use crate::domain::client::service::ClientService;
use crate::domain::statement::model::{Statement, StatementBalance, StatementTransaction};
use crate::domain::transaction::model::Transaction;
use crate::domain::transaction::service::TransactionService;
use crate::tools::db::Database;
use crate::tools::error::CustomError;

#[derive(new)]
pub struct StatementService {
    client_service: Arc<ClientService>,
    transaction_service: Arc<TransactionService>,
    db: Arc<dyn Database>,
}

impl StatementService {
    pub async fn find(&self, client_id: u32) -> Result<Statement, CustomError> {
        let tx = self
            .db
            .transaction(ReadOnly)
            .await
            .context("failed to start a transaction")?;

        let client = self.client_service.find(client_id, Some(&tx)).await?;
        let transactions = self
            .transaction_service
            .find_latest(client_id, Some(&tx))
            .await?;

        let balance = StatementBalance::new(
            client.balance,
            client.negative_limit,
            OffsetDateTime::now_utc(),
        );
        let statement_transactions = transactions
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<_>>();

        tx.commit().await.context("failed to commit transaction")?;

        Ok(Statement::new(balance, statement_transactions))
    }
}

impl From<Transaction> for StatementTransaction {
    fn from(val: Transaction) -> Self {
        StatementTransaction {
            amount: val.amount,
            operation: val.operation,
            description: val.description,
            created_at: val.created_at,
        }
    }
}
