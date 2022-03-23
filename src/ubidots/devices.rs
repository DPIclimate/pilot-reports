//! Get list of devices from ubidots.
//! This list reflects the org token used (reccommended that you use the global org key to get all
//! devices)
use log::info;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// URL for a devices data
    pub url: String,
    /// API_ID of a device
    pub id: String,
    /// API_Label of a device
    pub label: String,
    /// Name of the device
    pub name: String,
    /// When the last value came in from the device (UNIX time)
    pub last_activity: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    /// Number of results
    pub count: i64,
    /// List of devices
    pub results: Vec<Device>,
}

/// Get all devices within a organisation
///
/// Returns a Devices struct containg a vector of devices. This is used to compare
/// against devices in `config.json` thus allowing a curated list of devices
/// to be formulated.
#[tokio::main]
pub async fn get_all_devices(token: &String) -> Result<Devices, Box<dyn Error>> {
    info!("Getting device list from Ubidots");

    let url = String::from("https://industrial.api.ubidots.com.au/api/v2.0/devices/");

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
