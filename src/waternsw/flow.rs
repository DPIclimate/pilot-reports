use crate::utils;
use serde::{Deserialize, Serialize};
use std::error::Error;

// ---- Start of request body ---- //
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    params: Params,
    function: String,
    version: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Params {
    #[serde(rename = "site_list")]
    site_list: i64,
    #[serde(rename = "start_time")]
    start_time: i64,
    varfrom: f64,
    interval: String,
    varto: f64,
    datasource: String,
    #[serde(rename = "end_time")]
    end_time: i64,
    #[serde(rename = "data_type")]
    data_type: String,
    multiplier: i64,
}
// ---- End of request body ---- //

/// Contains response from discharge rate request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DischargeRate {
    /// Result from request
    pub result: Vec<Traces>,
}

/// Contains site information and all traces from response
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traces {
    /// Latitude of the measurement site
    pub latitude: String,
    /// Longitude of the measurement site
    pub longitude: String,
    /// Site name
    pub site: String,
    /// Result from request
    pub traces: Vec<Trace>,
}

/// Contains a single value, timestamp and quality code for a measurement
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    /// Volume of water per unit time (defaults to ML per day)
    pub v: String,
    /// Timestring (NOT TIMESTAMP!) e.g. 20191102000000 = 2019/11/02 00:00:00
    pub t: i64,
    /// Quality of the data (should be 140)
    pub q: i64,
}

impl DischargeRate {
    #[tokio::main]
    pub async fn new(
        (start, end): &(String, String),
        config: &utils::config::Config,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut discharge_rate_vec: Vec<DischargeRate> = Vec::new();
        for site in &config.water_nsw.sites {
            let params = Params {
                site_list: site.id,
                start_time: start.parse::<i64>().unwrap(),
                varfrom: config.water_nsw.defaults.params.varfrom.to_owned(),
                interval: config.water_nsw.defaults.params.interval.to_owned(),
                varto: config.water_nsw.defaults.params.varto.to_owned(),
                datasource: config.water_nsw.defaults.params.datasource.to_owned(),
                end_time: end.parse::<i64>().unwrap(),
                data_type: config.water_nsw.defaults.params.data_type.to_owned(),
                multiplier: config.water_nsw.defaults.params.multiplier.to_owned(),
            };

            let request = Request {
                params: params,
                function: config.water_nsw.defaults.function.to_owned(),
                version: config.water_nsw.defaults.version,
            };

            let req_str = serde_json::to_string(&request)?;

            let url = format!(
                "https://realtimedata.waternsw.com.au/cgi/webservice.exe?{}",
                req_str
            );

            let client = reqwest::Client::new();
            let response = client
                .get(url)
                .send()
                .await?
                .json::<DischargeRate>()
                .await?;

            discharge_rate_vec.push(response);
        }

        Ok(discharge_rate_vec)
    }
}
