use reqwest::blocking::Client;
use reqwest::Error;
use crate::tsdb::TsdbStatus;
use crate::AppState;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
use prometheus_client::encoding::text::encode;
use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Method {
    Get,
    Post,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MethodLabels {
    pub method: Method,
}

#[derive(Debug)]
pub struct Metrics {
    requests: Family<MethodLabels, Counter>,
}


pub fn initialize_registry(mut app_state: AppState) -> Result<AppState, Error>{
    let metrics = Metrics {
        requests: Family::default(),
    };
    app_state.registry.register("requests", "Count of requests", metrics.requests.clone());
    return Ok(app_state)
}

pub fn fetch_metrics(url: &str) -> Result<TsdbStatus, Error> {
    let client = Client::new();

    let endpoint = format!("{}/api/v1/status/tsdb", url);
    
    let response = client.get(&endpoint).send()?;
    
    let tsdb_status: TsdbStatus = response.json()?;

    Ok(tsdb_status)
}

pub async fn metrics_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(Body::from(buffer))
        .unwrap()
}


