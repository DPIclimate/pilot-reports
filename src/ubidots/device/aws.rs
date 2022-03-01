//! Get data from local weather station
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WeatherStation {
    pub results: Vec<Precipitation>,
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Precipitation {
    pub timestamp: i64,
    pub value: f64,
}


#[tokio::main]
pub async fn weekly_precipitation(token: &String) -> Result<WeatherStation, Box<dyn Error>>{

    // clyde-j301 (hardcoded here but could be dynamic)
    let device_label = String::from("00d646ad8b0c16d0"); 
    let variable_label = String::from("daily_precip_total_9am-9am");

    // Hard coded past 7 days of precip data
    let url =
        format!("https://industrial.api.ubidots.com/api/v1.6/devices/{}/{}/values/?format=json&page_size={}", device_label, variable_label, 7);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Auth-Token", token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<WeatherStation>()
        .await?;

    Ok(response)
}

