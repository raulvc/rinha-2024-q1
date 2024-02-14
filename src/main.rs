use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing_subscriber::fmt as log_guard;

use crate::state::State;

mod config;
mod domain;
mod routes;
mod state;
mod tools;

#[tokio::main]
async fn main() {
    log_guard().init();

    let state = State::new().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.server.port));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, routes::new(state)).await.unwrap();
}
