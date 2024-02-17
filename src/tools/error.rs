use std::error::Error;
use std::fmt::{Display, Formatter};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_new::new;
use serde::Serialize;
use validify::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("{0}")]
    Unexpected(#[from] anyhow::Error),

    #[error("{0}")]
    Validation(#[from] ValidationErrors),

    #[error("{0}")]
    Domain(#[from] DomainError),

    #[error("request is invalid: {0}")]
    Rejection(String),

    #[error("lock is being held: {0}")]
    LockHeld(String),
}

#[derive(new, Debug, Serialize)]
pub struct DomainError {
    pub message: String,
    pub status: u16,
}

impl DomainError {
    pub fn pretty(&self) -> String {
        format!("[{}] - {}", self.status, self.message)
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

impl Error for DomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// logs out and returns an error json payload for custom errors
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        match self {
            CustomError::Unexpected(err) => {
                tracing::error!("unexpected error: {} - {}", err, err.root_cause());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    r#"{ "code": "unexpected", "message": "erro inesperado", "status": 500 }"#,
                )
                    .into_response()
            }

            CustomError::Validation(err) => {
                tracing::warn!("{}", err);

                (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
            }

            CustomError::Domain(err) => {
                tracing::warn!("domain error: {}", err.pretty());

                (StatusCode::from_u16(err.status).unwrap(), Json(err)).into_response()
            }

            CustomError::Rejection(err) => {
                tracing::warn!("{}", err);

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    r#"{ "code": "unproc", "message": "requisição inválida", "status": 422 }"#,
                )
                    .into_response()
            }

            CustomError::LockHeld(err) => {
                tracing::warn!(err);

                (
                    StatusCode::LOCKED,
                    r#"{ "code": "lockheld", "message": "recurso indisponível", "status": 409 }"#,
                )
                    .into_response()
            }
        }
    }
}
