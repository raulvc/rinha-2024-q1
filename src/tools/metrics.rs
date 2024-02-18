use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{register_histogram_vec, HistogramVec};

lazy_static! {
    pub static ref OPS_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "ops_duration_seconds",
        "Performance histograms (uses seconds)",
        &["operation"]
    )
    .unwrap();
}

pub fn register(registry: &prometheus::Registry) {
    registry.register(Box::new(OPS_HISTOGRAM.clone())).unwrap();
}

pub async fn get(State(registry): State<Arc<prometheus::Registry>>) -> impl IntoResponse {
    let mut buffer = vec![];
    let encoder = prometheus::TextEncoder::new();
    let registered_metrics = registry.gather();
    prometheus::Encoder::encode(&encoder, &registered_metrics, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
