//! Get device variables
use serde::Deserialize;
use std::error::Error;

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

pub struct VariablesList {
    pub name: String,
    pub ids: Vec<String>,
    pub corresponding_device: Vec<String>,
    pub harvest_area: Vec<String>,
}

impl VariablesList {
    // Contains a list of like varaible ids to use in request to Ubidots

    pub fn add_variable_and_device(&mut self, variable_id: &String, device_name: &String, harvest_area: &String) {
        self.ids.push(variable_id.to_string());
        self.corresponding_device.push(device_name.to_string());
        self.harvest_area.push(harvest_area.to_string());
    }
}

