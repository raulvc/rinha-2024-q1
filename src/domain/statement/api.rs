use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::domain::statement::model::Statement;
use crate::domain::statement::service::StatementService;
use crate::tools::axum::Path;
use crate::tools::error::CustomError;

#[tracing::instrument(skip_all, fields(client_id = % client_id))]
pub async fn find_statement(
    State(statement_service): State<Arc<StatementService>>,
    Path(client_id): Path<u32>,
) -> Result<Json<Statement>, CustomError> {
    let response = statement_service.find(client_id).await?;

    Ok(Json(response))
}
