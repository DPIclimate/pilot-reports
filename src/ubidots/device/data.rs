//! Device data by aggrigation (weekly or fortnightly)
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

// This represents the body that needs to be sent to Ubidots when reqesting data aggregation.
#[derive(Serialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub results: Vec<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value: Option<f64>,
    pub timestamp: i64,
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resample {
    /// Variable API labels (not names unfortunatly)
    pub variables: Vec<String>,
    /// mean, min, max, sum, count
    pub aggregation: String,
    /// Combined to single value (should be false for this request)
    #[serde(rename = "join_dataframes")]
    pub join_dataframes: bool,
    /// resampling periods => nT, nH, nD, W, M => (minutes, hours, days, week, month)
    pub period: String,
    /// Start of aggregation
    pub start: i64,
    /// End of aggregation
    pub end: i64,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResampleResult {
    /// Resampled values contains timestamp as the first value
    pub results: Vec<Vec<Option<f64>>>,
    /// Column names for items inside results
    pub columns: Vec<String>,
}

impl Resample {
    /// Request resampled data from ubidots based on `Resample` body.
    #[tokio::main]
    pub async fn resample(&self, token: &String) -> Result<ResampleResult, Box<dyn Error>> {
        info!("Getting resampled data from ubidots.");
        let url =
            String::from("https://industrial.api.ubidots.com.au/api/v1.6/data/stats/resample/");

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Auth-Token", token)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?) // Convert struct to JSON
            .send()
            .await?;

        let json_res = response.json::<ResampleResult>().await?;

        Ok(json_res)
    }
}
