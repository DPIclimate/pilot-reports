//! Export data to an email address
use serde::Deserialize;
use std::error::Error;
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
    (start, end): &(i64, i64)) -> Result<i32, Box<dyn Error>>{
    // Export all devices to email within a set date range
    // Ubidots requires a post request to get an email containing a .csv file of device data
    // This function triggers the email to be sent for all devices within a device
    // The email has to be handled hence the use of a gmail module

    let mut ok_responses = 0; // Tracks number of successful requests

    for dev in &devices.results {
        let api_label = device::Identifier::ApiLabel(String::from(&dev.label));

        println!("{:#?}", &dev.label);

        let url = match api_label {
            device::Identifier::ApiLabel(l) => {
                format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/_/values/export/?email={}&startDate={}&endDate={}&timezone=Australia/Sydney", l, email, start, end)
            },
            device::Identifier::Id(i) => {
                format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/_/values/export/?email={}&startDate={}&endDate={}&timezone=Australia/Sydney", i, email, start, end)
            }
        };
        
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .header("X-Auth-Token", token)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        println!("Response status: {}", &res.status());

        ok_responses += 1;
    }

    Ok(ok_responses)
}

