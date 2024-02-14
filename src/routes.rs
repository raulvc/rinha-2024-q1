use axum::routing::{get, post, IntoMakeService};
use axum::Router;

use crate::domain::statement::api::find_statement;
use crate::domain::transaction::api::create_transaction;
use crate::state::State;

pub(crate) fn new(state: State) -> IntoMakeService<Router> {
    Router::new()
        .route("/clientes/:client_id/transacoes", post(create_transaction))
        .route("/clientes/:client_id/extrato", get(find_statement))
        .with_state(state)
        .into_make_service()
}
