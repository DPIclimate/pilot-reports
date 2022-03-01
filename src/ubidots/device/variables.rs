//! Get device variables
use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::utils;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub count: i64,
    pub results: Vec<Variable>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub last_activity: i64,
}

#[tokio::main]
pub async fn list_variables(device_id: &String, token: &String) -> Result<Variables, Box<dyn Error>> {
    // Lists all variables for a device
    let url = format!("https://industrial.api.ubidots.com/api/v2.0/devices/{}/variables/", device_id);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Auth-Token", token)
        .send()
        .await?
        .json::<Variables>()
        .await?;

    Ok(response)
}

