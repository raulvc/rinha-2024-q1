use std::sync::Arc;
use std::time::Instant;

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

pub struct DeferredObserve<'a> {
    histogram: &'a HistogramVec,
    label: &'a [&'a str],
    start_time: Instant,
}

impl<'a> DeferredObserve<'a> {
    pub fn new(histogram: &'a HistogramVec, label: &'a [&'a str]) -> Self {
        Self {
            histogram,
            label,
            start_time: Instant::now(),
        }
    }
}

impl<'a> Drop for DeferredObserve<'a> {
    fn drop(&mut self) {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        self.histogram
            .with_label_values(self.label)
            .observe(elapsed_time);
    }
}
