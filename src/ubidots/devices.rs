//! Get device lists 
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub url: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub last_activity: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    pub count: i64,
    pub results: Vec<Device>,
}

#[tokio::main]
pub async fn get_all_devices(token: &String) -> Result<Devices, Box<dyn Error>> {
    // Get all devices within a organisation 
    // Returns a Devices struct containg a vector of devices
    let url = String::from("https://industrial.api.ubidots.com/api/v2.0/devices/");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Auth-Token", token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Devices>()
        .await?;

    Ok(response)
}

