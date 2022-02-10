//! Get a single device from ubidots
use serde::Deserialize;
use std::error::Error;

pub mod variables;

pub enum Identifier {
    ApiLabel(String),
    Id(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub url: String,
    pub id: String,
    //pub organization: Organization,
    pub label: String,
    pub name: String,
    pub description: String,
    //pub tags: Vec<Value>,
    //pub properties: Properties,
    pub is_active: bool,
    pub last_activity: i64,
    pub created_at: String,
    //pub location: Location,
    pub variables: String,
    pub variables_count: i64, 
}

#[tokio::main]
pub async fn get(id: &Identifier, token: &String) -> Result<Device, Box<dyn Error>> {
    let url = match id {
        Identifier::ApiLabel(l) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}", l)
        },
        Identifier::Id(i) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/{}", i)
        }
    };

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Auth-Token", token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Device>()
        .await?;

    Ok(response)
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    pub count: i64,
    pub results: Vec<Device>,
}


#[tokio::main]
pub async fn get_all(token: &String) -> Result<Devices, Box<dyn Error>> {
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
