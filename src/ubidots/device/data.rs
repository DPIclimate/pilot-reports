//! Device data by aggrigation (weekly or fortnightly)
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

// This represents the body that needs to be sent to Ubidots when reqesting data aggregation.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregation {
    /// Variable API labels (not names unfortunatly)
    pub variables: Vec<String>,
    /// mean, min, max, sum, count
    pub aggregation: String,
    /// Combined to single value (should be false for this request)
    #[serde(rename = "join_dataframes")]
    pub join_dataframes: bool,
    /// Start of aggregation
    pub start: i64,
    /// End of aggregation (defaults to Unix time now if ommitted)
    pub end: i64,
}

impl Aggregation {
    /// Request aggregate data from ubidots based on `Aggregation` body.
    #[tokio::main]
    pub async fn aggregate(&self, token: &String) -> Result<Response, Box<dyn Error>> {
        info!("Getting aggregate data from ubidots.");
        let url =
            String::from("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/aggregation/");

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Auth-Token", token)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?) // Convert struct to JSON
            .send()
            .await?;

        let json_res = response.json::<Response>().await?;

        Ok(json_res)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub results: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value: f64,
    pub timestamp: i64,
}
