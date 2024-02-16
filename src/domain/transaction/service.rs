use std::sync::Arc;

use anyhow::Context;
use axum::http::StatusCode;
use derive_new::new;
use libsql::de;
use sea_query::{Expr, Order, Query, SqliteQueryBuilder};

use crate::domain::client::service::ClientService;
use crate::domain::transaction::model::{
    CreateTransactionRequest, CreateTransactionResponse, Transaction, TransactionTable,
    OPERATION_CREDIT,
};
use crate::tools::db::Database;
use crate::tools::error::{CustomError, DomainError};
use crate::tools::locker::Locker;

#[derive(new)]
pub struct TransactionService {
    client_service: Arc<ClientService>,
    db: Arc<dyn Database>,
    locker: Arc<Locker>,
}

impl TransactionService {
    pub async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<CreateTransactionResponse, CustomError> {
        let _client = self.client_service.find(request.client_id).await?;

        let key = format!("transaction:{}", request.client_id);

        self.locker
            .with_lock(key, || self.process_new_transaction(request))
            .await
    }

    pub async fn find_latest(
        &self,
        client_id: u32,
        conn: Option<&dyn Database>,
    ) -> Result<Vec<Transaction>, CustomError> {
        let db = conn.unwrap_or(&*self.db);
        let query = Self::find_latest_query(client_id);

        let mut rows = db.query(&query).await.context("failed to query for rows")?;

        let mut transactions = Vec::new();
        while let Some(row) = rows.next().await.context("failed to retrieve next row")? {
            let transaction = de::from_row::<Transaction>(&row).context("failed to parse row")?;
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    async fn process_new_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<CreateTransactionResponse, CustomError> {
        let client_id = request.client_id;
        let meta = self.client_service.find_meta(client_id, None).await?;
        let new_balance = Self::calculate_new_balance(meta.balance, &request);

        if new_balance < -meta.negative_limit {
            return Err(DomainError::new(
                format!("Insufficient funds for client {}", client_id),
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            ))?;
        }

        self.persist(request, new_balance).await?;

        Ok(CreateTransactionResponse::new(
            meta.negative_limit,
            new_balance,
        ))
    }

    fn calculate_new_balance(current_balance: i32, request: &CreateTransactionRequest) -> i32 {
        let amount = if request.payload.operation == OPERATION_CREDIT {
            request.payload.amount
        } else {
            -request.payload.amount
        };

        current_balance + amount
    }

    async fn persist(
        &self,
        request: CreateTransactionRequest,
        new_balance: i32,
    ) -> Result<(), CustomError> {
        let balance_update = ClientService::balance_update_query(request.client_id, new_balance);
        let transaction_insert = Self::insert_query(request);

        let statements = [balance_update, transaction_insert].join(";");

        self.db
            .execute_batch(&statements)
            .await
            .context("failed to persist new transaction")?;

        Ok(())
    }

    fn insert_query(request: CreateTransactionRequest) -> String {
        Query::insert()
            .into_table(TransactionTable::Table)
            .columns([
                TransactionTable::ClientID,
                TransactionTable::Amount,
                TransactionTable::Operation,
                TransactionTable::Description,
            ])
            .values_panic([
                request.client_id.into(),
                request.payload.amount.into(),
                request.payload.operation.into(),
                request.payload.description.into(),
            ])
            .to_string(SqliteQueryBuilder)
            .to_owned()
    }

    fn find_latest_query(client_id: u32) -> String {
        Query::select()
            .columns([
                TransactionTable::ClientID,
                TransactionTable::Amount,
                TransactionTable::Operation,
                TransactionTable::Description,
                TransactionTable::CreatedAt,
            ])
            .from(TransactionTable::Table)
            .and_where(Expr::col(TransactionTable::ClientID).eq(client_id))
            .order_by(TransactionTable::CreatedAt, Order::Desc)
            .limit(10)
            .to_string(SqliteQueryBuilder)
            .to_owned()
    }
}
