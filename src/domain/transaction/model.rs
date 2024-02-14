use crate::tools::json::deserialize_sqlite_timestamp;
use derive_new::new;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validify::Validate;

pub const OPERATION_CREDIT: &str = "c";
pub const OPERATION_DEBIT: &str = "d";
const OPERATIONS: &[&str] = &[OPERATION_CREDIT, OPERATION_DEBIT];

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    pub client_id: u32,
    pub amount: i32,
    pub operation: String,
    pub description: String,

    #[serde(deserialize_with = "deserialize_sqlite_timestamp")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateTransactionPayload {
    #[serde(rename = "valor")]
    #[validate(range(min = 1.))]
    pub amount: i32,

    #[serde(rename = "tipo")]
    #[validate(is_in(OPERATIONS))]
    pub operation: String,

    #[serde(rename = "descricao")]
    #[validate(length(min = 1, max = 10))]
    pub description: String,
}

#[derive(Debug, Clone, new)]
pub struct CreateTransactionRequest {
    pub client_id: u32,
    pub payload: CreateTransactionPayload,
}

#[derive(Debug, Serialize, new)]
pub struct CreateTransactionResponse {
    #[serde(rename = "limite")]
    pub negative_limit: i32,

    #[serde(rename = "saldo")]
    pub balance: i32,
}

#[derive(Copy, Clone, Iden, PartialEq)]
pub enum TransactionTable {
    #[iden = "transactions"]
    Table,
    ClientID,
    Amount,
    Operation,
    Description,
    CreatedAt,
}
