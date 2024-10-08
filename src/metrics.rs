use prometheus_client::registry::Registry;
use reqwest::Client;
use reqwest::Error;
use crate::config::Config;
use crate::tsdb::TsdbStatus;
use crate::AppState;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::encoding::text::encode;
use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DefaultLabels {
    pub instance: String,
    pub name: String,
}

#[derive()]
pub struct Metrics {
    status: Family<DefaultLabels, Gauge>,
    series_count_by_metric_name: Family<DefaultLabels, Gauge>,
    series_count_by_label_name: Family<DefaultLabels, Gauge>,
    memory_in_bytes_by_label_name: Family<DefaultLabels, Gauge>,
}


pub fn initialize_registry(mut registry: Registry, config: Config) -> Result<AppState, Error>{
    let metrics = Metrics {
        status: Family::default(),
        series_count_by_metric_name: Family::default(),
        series_count_by_label_name: Family::default(),
        memory_in_bytes_by_label_name: Family::default(),
    };

    registry.register("status", "TSDB Status", metrics.status.clone());
    registry.register("series_count_by_metric_name", "Series Count by Metric Name", metrics.series_count_by_metric_name.clone());
    registry.register("series_count_by_label_name", "Series Count by Metric Name", metrics.series_count_by_label_name.clone());
    registry.register("memory_in_bytes_by_label_name", "Memory in Bytes by Label Name", metrics.memory_in_bytes_by_label_name.clone());

    return Ok(AppState{ metrics, registry, config})
}

async fn fetch_metrics(url: &str) -> Result<TsdbStatus, Error> {
    let client = Client::new();

    let endpoint = format!("{}/api/v1/status/tsdb", url);
    
    let response = client.get(&endpoint).send().await?;
    
    let tsdb_status: TsdbStatus = response.json().await?;

    Ok(tsdb_status)
}

pub async fn metrics_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;

    for instance in &state.config.prometheus.instances {
        match fetch_metrics(&instance.url).await {
            Ok(metrics) => {
                state.metrics.status.get_or_create(&DefaultLabels{instance: instance.name.clone(), name: "status".to_string()}).set(0);
                if metrics.status == "success" {
                    state.metrics.status.get_or_create(&DefaultLabels{instance: instance.name.clone(), name: "status".to_string()}).set(0);
                }

                for metric in &metrics.data.series_count_by_metric_name {
                    let default_labels = &DefaultLabels{
                        instance: instance.name.clone(),
                        name: metric.name.clone(),
                    };
                    state.metrics.series_count_by_metric_name.get_or_create(default_labels).set(metric.value);
                }

                for metric in &metrics.data.series_count_by_label_name {
                    let default_labels = &DefaultLabels{
                        instance: instance.name.clone(),
                        name: metric.name.clone(),
                    };
                    state.metrics.series_count_by_label_name.get_or_create(default_labels).set(metric.value);
                }

                for metric in &metrics.data.memory_in_bytes_by_label_name {
                    let default_labels = &DefaultLabels{
                        instance: instance.name.clone(),
                        name: metric.name.clone(),
                    };
                    state.metrics.memory_in_bytes_by_label_name.get_or_create(default_labels).set(metric.value);
                }

            }
            Err(e) => {
                eprint!("Error fetching metrics from instance {}: {}", &instance.name, e)
            }
        }; 
    };

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


