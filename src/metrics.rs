use reqwest::blocking::Client;
use reqwest::Error;
use crate::tsdb::TsdbStatus;

pub fn fetch_metrics(url: &str) -> Result<TsdbStatus, Error> {
    let client = Client::new();

    let endpoint = format!("{}/api/v1/status/tsdb", url);
    
    let response = client.get(&endpoint).send()?;
    
    let tsdb_status: TsdbStatus = response.json()?;

    Ok(tsdb_status)
}
