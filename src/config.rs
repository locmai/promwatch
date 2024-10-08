use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prometheus: PrometheusInstances,
    pub export: ExportDestination,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusInstances {
    pub instances: Vec<PrometheusInstance>,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusInstance {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportDestination {
    pub destination: PrometheusInstance,
}
