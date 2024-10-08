use std::fs;
use clap::Parser;
use serde_yaml;
use crate::config::Config;
use tokio::time::{self, Duration};

use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use prometheus_client::encoding::{EncodeLabelValue,EncodeLabelSet};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::counter::Counter;
use std::sync::Arc;
use tokio::sync::Mutex;

use metrics::fetch_metrics;

mod tsdb;
mod metrics;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "promwatch-config.yaml")]
    config: String,
}

fn load_config(filename: &str) -> Config {
    let config_content = fs::read_to_string(filename)
        .expect("Failed to read the configuration file");

    serde_yaml::from_str(&config_content)
        .expect("Failed to parse the configuration file")
}

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

#[derive(Debug)]
pub struct AppState {
    pub registry: Registry,
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);
    println!("Loaded configuration: {:?}", config);
    
    //for instance in config.prometheus.instances {
    //    tokio::spawn(async move {
    //        let mut interval = time::interval(Duration::from_secs(15));
    //        loop {
    //            interval.tick().await;
    //            match fetch_metrics(&instance.url) {
    //                Ok(metrics) => {
    //                    println!("Metrics from {}: {}", &instance.name.to_string(), metrics.status);
    //                    println!("Metrics from {}: {}", &instance.name.to_string(), metrics.data.series_count_by_metric_name[0].name);
    //
    //                }
    //                Err(e) => {
    //                    eprintln!("Error fetching metrics from {}: {}", &instance.name, e);
    //                }
    //            }
    //        }
    //    });
    //};

    let metrics = Metrics {
        requests: Family::default(),
    };
    let mut state = AppState {
        registry: Registry::default(),
    };
    state
        .registry
        .register("requests", "Count of requests", metrics.requests.clone());
    let state = Arc::new(Mutex::new(state));
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);
    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
