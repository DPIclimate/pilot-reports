//! Get data from local weather station
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
pub struct Precipitation {
    /// Nested values
    pub results: Vec<Vec<Vec<f64>>>,
    /// Corresponding columns for values
    pub columns: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSeries {
    /// List of variables to gather raw data from
    pub variables: Vec<String>,
    /// Type of data to return (primarily "value.value" and "timestamp")
    pub columns: Vec<String>,
    /// Join data to a single global value
    #[serde(rename = "join_dataframes")]
    pub join_dataframes: bool,
    /// Start of raw series
    pub start: i64,
    /// End of raw series. Defaults to now if end is not provided.
    pub end: i64,
}

impl RawSeries {
    /// New raw series body for Ubidots request.
    ///
    /// See <https://docs.ubidots.com/v1.6/reference/data-raw-series> for docs.
    ///
    /// Base POST request to: <https://industrial.api.ubidots.com/api/v1.6/data/raw/series>
    pub fn new(variables: &Vec<String>, (start, end): (i64, i64)) -> Self {
        RawSeries {
            variables: variables.to_owned(),
            columns: vec!["value.value".to_string(), "timestamp".to_string()],
            join_dataframes: false,
            start: start,
            end: end,
        }
    }

    /// Get precipitation from AWS based on RawSeries body.
    ///
    /// Returns a nested list of `Results` which has to be unwrapped and parsed.;
    #[tokio::main]
    pub async fn get_precipitation(
        &self,
        aws_token: &String,
    ) -> Result<Precipitation, Box<dyn Error>> {
        let url = "https://industrial.api.ubidots.com.au/api/v1.6/data/raw/series";

        info!("Getting precipitation");

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Auth-Token", aws_token)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?) // Convert struct to JSON
            .send()
            .await?
            .json::<Precipitation>()
            .await?;

        Ok(response)
    }
}
