//! Get device variables
use crate::ubidots;
use crate::utils;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::{thread, time};

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
pub async fn list_variables(
    device_id: &String,
    token: &String,
) -> Result<Variables, Box<dyn Error>> {
    // Lists all variables for a device

    info!("Getting variables list from Ubidots.");

    let url = format!(
        "https://industrial.api.ubidots.com/api/v2.0/devices/{}/variables/",
        device_id
    );

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

#[derive(Deserialize, Serialize)]
pub struct VariablesList {
    pub name: String,
    pub ids: Vec<String>,
    pub corresponding_device: Vec<String>,
    pub harvest_area: Vec<String>,
}

impl VariablesList {
    pub fn new(variable: &String, config: &utils::config::Config, token: &String) -> Self {
        let mut variable_list = VariablesList {
            name: variable.to_string(),
            ids: Vec::new(),
            corresponding_device: Vec::new(),
            harvest_area: Vec::new(),
        };

        info!("Getting refined variable list from config.json");

        // Get all devcies from Ubidots under specific org
        let all_devices = ubidots::devices::get_all_devices(&token)
            .map_err(|err| error!("Error getting devices list: {}", err))
            .ok()
            .unwrap();

        for device in &all_devices.results {
            if config.devices.iter().any(|dev| &dev.name == &device.name) {
                thread::sleep(time::Duration::from_secs(3));

                let all_variables = list_variables(&device.id, &token)
                    .map_err(|err| error!("Error getting device variables: {}", err))
                    .ok()
                    .unwrap();

                // Check if variables are contained within the requested variables (config.json)
                for var in &all_variables.results {
                    if &var.name == variable {
                        let mut location: String = "unknown".to_string();
                        let mut harvest_area: String = "unknown".to_string();
                        for dev in &config.devices {
                            if &dev.name == &device.name {
                                location = dev.location.to_owned();
                                harvest_area = dev.harvest_area.to_owned();
                                break;
                            }
                        }
                        variable_list.add_variable_and_device(&var.id, &location, &harvest_area);
                        break;
                    }
                }
            }
        }
        variable_list
    }

    pub fn new_from_cache(variable: &String) -> VariablesList {
        let filename = format!("cache/{}-variable-list.json", variable);
        let file = File::open(filename).expect("Unable to open variable-list from cache");
        serde_json::from_reader(file).expect("Error reading variable-list from cache")
    }

    pub fn add_variable_and_device(
        &mut self,
        variable_id: &String,
        device_name: &String,
        harvest_area: &String,
    ) {
        self.ids.push(variable_id.to_string());
        self.corresponding_device.push(device_name.to_string());
        self.harvest_area.push(harvest_area.to_string());
    }

    pub fn cache(&self, variable: &String) {
        let filename = format!("cache/{}-variable-list.json", variable);
        let file =
            File::create(filename).expect("Unable to create {variable_name}-variable-list.json");
        serde_json::to_writer_pretty(&file, &self)
            .expect("Error parsing devices to variable_list.json");
    }
}
