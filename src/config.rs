use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prometheus: PrometheusInstances,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusInstances {
    pub instances: Vec<PrometheusInstance>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PrometheusInstance {
    pub name: String,
    pub url: String,
    #[serde(default = "default_limit")]
    pub limit: i32
}

// Function to provide default value
fn default_limit() -> i32 {
    10
}
