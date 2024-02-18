use axum::routing::{get, post, IntoMakeService};
use axum::Router;
use tower_http::catch_panic::CatchPanicLayer;

use crate::domain::statement::api::find_statement;
use crate::domain::transaction::api::create_transaction;
use crate::state::State;
use crate::tools::error::handle_panic;

pub(crate) fn new(state: State) -> IntoMakeService<Router> {
    Router::new()
        .route("/health", get(|| async { "OK" })) // used by haproxy
        .route("/clientes/:client_id/transacoes", post(create_transaction))
        .route("/clientes/:client_id/extrato", get(find_statement))
        .with_state(state)
        .layer(CatchPanicLayer::custom(handle_panic))
        .into_make_service()
}
