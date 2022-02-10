//! Get variables for a device
use serde::Deserialize;
use std::error::Error;
use crate::ubidots::device;

pub mod values;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub count: i64,
    pub results: Vec<Variable>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub url: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub description: String,
    pub device: ShortDevice,
    //pub tags: Vec<Value>,
    //pub properties: Properties,
    #[serde(rename = "type")]
    pub type_field: String,
    //pub unit: Value,
    //pub synthetic_expression: String,
    pub created_at: String,
    //pub last_value: LastValue,
    pub last_activity: i64,
    pub values_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortDevice {
    pub url: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub created_at: String,
}

#[tokio::main]
pub async fn get(id: &device::Identifier, token: &String) -> Result<Variables, Box<dyn Error>> {
    // Gets all the variables of a device (using the device id)
    // This method isn't really nessessary but may be used in the future
    let url = match id {
        device::Identifier::ApiLabel(l) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/variables", l)
        },
        device::Identifier::Id(i) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/{}/variables", i)
        }
    };

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Auth-Token", token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Variables>()
        .await?;

    Ok(response)
}
