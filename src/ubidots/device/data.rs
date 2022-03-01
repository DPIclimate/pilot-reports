//! Device data by aggrigation (weekly or fortnightly)
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::utils;

// This represents the body that needs to be sent to Ubidots when reqesting data aggregation.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregation {
    pub variables: Vec<String>, // Variable API labels (not names unfortunatly)
    pub aggregation: String, // mean, min, max, sum, count
    #[serde(rename = "join_dataframes")]
    pub join_dataframes: bool, // should be false
    pub start: i64,
    pub end: i64,
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


impl Aggregation {
    #[tokio::main]
    pub async fn aggregate(&self, token: &String) -> Result<Response, Box<dyn Error>> {

        let url = String::from("https://industrial.api.ubidots.com/api/v1.6/data/stats/aggregation/");

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Auth-Token", token)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?) // Convert struct to JSON
            .send() 
            .await?;

        println!("Aggregate response: {}", &response.status());

        let json_res = response
            .json::<Response>()
            .await?;

        Ok(json_res)
    }
}

#[derive(Serialize)]
struct Record<'a>{
    day_of_week: &'a String, 
    value: &'a f64,
}

impl Response {
    pub fn to_csv(&self, variable_name: &String) {

        let file = format!("data/{}.csv", variable_name);
        let mut wtr = csv::Writer::from_path(file)
            .expect("Unable to write to request file path / filename");

        // Should only be one result
        for res in &self.results {
            let local_day = utils::time::unix_to_local(&res.timestamp)
                .date()
                .format("%A");
            let rec = Record {
                day_of_week: &local_day.to_string(),
                value: &res.value
            };
            wtr.serialize(rec).expect("Unable to write to CSV");
        }

        wtr.flush().expect("Error flushing writer");
    }
}

