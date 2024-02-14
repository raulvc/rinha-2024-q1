use std::sync::Arc;

use axum::extract::FromRef;

use crate::config::app_config::AppConfig;
use crate::domain::client::service::ClientService;
use crate::domain::statement::service::StatementService;
use crate::domain::transaction::service::TransactionService;
use crate::tools::db;
use crate::tools::locker::Locker;

#[derive(Clone, FromRef)]
pub(crate) struct State {
    pub config: Arc<AppConfig>,
    pub locker: Arc<Locker>,
    pub transaction_service: Arc<TransactionService>,
    pub statement_service: Arc<StatementService>,
}

impl State {
    pub async fn new() -> Self {
        let config = Arc::new(AppConfig::new());
        let db = Arc::new(db::new(&config).await);
        let locker = Arc::new(Locker::new(&config).await);

        let client_service = Arc::new(ClientService::new(db.clone()));
        let transaction_service = Arc::new(TransactionService::new(
            client_service.clone(),
            db.clone(),
            locker.clone(),
        ));
        let statement_service = Arc::new(StatementService::new(
            client_service,
            transaction_service.clone(),
            db,
        ));

        State {
            config,
            locker,
            transaction_service,
            statement_service,
        }
    }
}
