//! Get device lists
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub url: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub last_activity: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    pub count: i64,
    pub results: Vec<Device>,
}

#[tokio::main]
pub async fn get_all_devices(token: &String) -> Result<Devices, Box<dyn Error>> {
    // Get all devices within a organisation
    // Returns a Devices struct containg a vector of devices

    info!("Getting device list from Ubidots");

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

impl Devices {
    pub fn cache(&self) {
        let file = File::create("cache/devices.json").expect("Unable to create devices.json");
        serde_json::to_writer_pretty(&file, &self).expect("Error parsing devices to devices.json");
    }
}
