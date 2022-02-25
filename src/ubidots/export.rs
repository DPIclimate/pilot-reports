//! Export data to an email address
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::ubidots::device;


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub task: PostId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostId {
    pub id: String,
}


#[tokio::main]
pub async fn data_to_email(token: &String, email: &String, devices: &device::Devices,
    (start, end): &(i64, i64)) -> Result<i64, Box<dyn Error>>{
    // Export all devices to email within a set date range
    // Ubidots requires a post request to get an email containing a .csv file of device data
    // This function triggers the email to be sent for all devices within a device
    // The email has to be handled hence the use of a gmail module
    // Returns i64 representing the number of successful requests

    println!("Exporting device data to {}", email);

    let buoys_list = get_selected_buoys()
        .map_err(|err| println!("{}", err))
        .ok().unwrap();

    let mut ok_responses = 0; // Tracks number of successful requests
    for dev in &devices.results {
        if buoys_list.buoys.iter().any(|b| &b.name == &dev.name){

            let api_label = device::Identifier::ApiLabel(String::from(&dev.label));

            let url = match api_label {
                device::Identifier::ApiLabel(l) => {
                    format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/_/values/export/?email={}&startDate={}&endDate={}&timezone=Australia/Sydney", l, email, start, end)
                }
            };
            
            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .header("X-Auth-Token", token)
                .header("Content-Type", "application/json")
                .send()
                .await?;

            println!("Response status: {} for device: {}", &res.status(), &dev.name);

            ok_responses += 1;
        }
    }

    Ok(ok_responses)
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buoys {
    pub buoys: Vec<Buoy>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buoy {
    pub name: String,
    pub location: String,
    #[serde(rename = "buoy_number")]
    pub buoy_number: String,
}


fn get_selected_buoys() -> Result<Buoys, Box<dyn Error>> {
    // Gets a list of buoys for which should be included in 
    // future processes.

    let file = File::open("devices.json")
        .expect("Error, devices.json file not found.");
    let reader = BufReader::new(file);
    let devices = serde_json::from_reader(reader)
        .expect("Error, device.json should be valid json.");

    Ok(devices)
}

