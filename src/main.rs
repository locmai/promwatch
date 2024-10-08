use std::fs;
use clap::Parser;
use serde_yaml;
use crate::config::Config;

use axum::routing::get;
use axum::Router;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;
use metrics::{initialize_registry, metrics_handler};

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

#[derive(Debug)]
pub struct AppState {
    pub registry: Registry,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);
    println!("Loaded configuration: {:?}", config);

    let mut state = AppState {
        registry: Registry::default(),
        config,
    };
    state = initialize_registry(state).unwrap(); 

    let state = Arc::new(Mutex::new(state));
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);
    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
