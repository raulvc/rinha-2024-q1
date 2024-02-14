use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use validify::Validate;

use crate::domain::transaction::model::{
    CreateTransactionPayload, CreateTransactionRequest, CreateTransactionResponse,
};
use crate::domain::transaction::service::TransactionService;
use crate::tools::axum::Path;
use crate::tools::error::CustomError;

#[tracing::instrument(skip_all, fields(client_id = % client_id))]
pub async fn create_transaction(
    State(transaction_service): State<Arc<TransactionService>>,
    Path(client_id): Path<u32>,
    Json(payload): Json<CreateTransactionPayload>,
) -> Result<Json<CreateTransactionResponse>, CustomError> {
    payload.validate()?;

    let request = CreateTransactionRequest::new(client_id, payload);
    let response = transaction_service.create_transaction(request).await?;

    tracing::info!("Transaction created successfully");

    Ok(Json(response))
}
