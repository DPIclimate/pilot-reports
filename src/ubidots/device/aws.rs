//! Get data from local weather station
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
pub struct Precipitation {
    pub results: Vec<Vec<Vec<f64>>>,
    pub columns: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSeries {
    pub variables: Vec<String>,
    pub columns: Vec<String>,
    #[serde(rename = "join_dataframes")]
    pub join_dataframes: bool,
    pub start: i64,
    pub end: i64,
}

impl RawSeries {
    pub fn new(variables: &Vec<String>, (start, end): (i64, i64)) -> Self {
        RawSeries {
            variables: variables.to_owned(),
            columns: vec!["value.value".to_string(), "timestamp".to_string()],
            join_dataframes: false,
            start: start,
            end: end,
        }
    }

    #[tokio::main]
    pub async fn get_year_to_date(
        &self,
        aws_token: &String,
    ) -> Result<Precipitation, Box<dyn Error>> {
        let url = "https://industrial.api.ubidots.com/api/v1.6/data/raw/series";

        info!("Getting year to date precipitation");

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
