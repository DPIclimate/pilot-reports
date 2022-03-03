//! Device data by aggrigation (weekly or fortnightly)
use serde::{Deserialize, Serialize};
use std::error::Error;

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

        print!("Aggregating data from ubidots...");

        let url = String::from("https://industrial.api.ubidots.com/api/v1.6/data/stats/aggregation/");

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Auth-Token", token)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?) // Convert struct to JSON
            .send() 
            .await?;

        let json_res = response
            .json::<Response>()
            .await?;

        println!("finished");

        Ok(json_res)
    }
}

