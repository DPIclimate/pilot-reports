use crate::utils;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub count: i64,
    pub data: Vec<Data>,
    pub download: bool,
    pub end: i64,
    pub start: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timestamp: i64,
    pub value: String,
}

#[derive(Serialize)]
struct Record<'a> {
    date: &'a String,
    precpitation: &'a f64,
}

impl Dataset {
    /*
     * Create a new dataset from IBM's timeseries forecast.
     * Leverages IBM's 30 km grid data to obtain both forecast and historical data for
     * a particular location.
     */
    #[tokio::main]
    pub async fn new(
        ibm_access_token: &String,
        layer_id: i32,
        lat: f64,
        lng: f64,
        start: &i64,
        end: &i64,
    ) -> Result<Self, Box<dyn Error>> {
        info!("Requesting precipitation data from IBM");

        let url: String = format!(
            "https://pairs.res.ibm.com/v2/timeseries?layer={}&lat={}&lon={}&start={}&end={}",
            layer_id, lat, lng, start, end
        );

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .bearer_auth(ibm_access_token)
            .send()
            .await?
            .json::<Dataset>()
            .await?;

        Ok(response)
    }

    /*
     * Data is returned from IBM on 3 hourly intervals. We want one value per day representing
     * total daily forcasted rainfall. This funciton aggregates the rainfall for the day and
     * appends a daily total precipitaiton value to a CSV file along with a timestamp.
     */
    pub fn aggregate_to_csv(&self) {
        if self.data.len() < 1 {
            error!("Precipitation dataset was empty.");
            return;
        }

        let file_path = String::from("data/forcast-precipitation.csv");

        let mut wtr = csv::Writer::from_path(file_path).expect("Unable to find csv to write to.");

        let mut current_day = utils::time::unix_to_local(&self.data[0].timestamp);
        let mut sum: f64 = 0.0;
        for value in self.data.iter() {
            let local_time = utils::time::unix_to_local(&value.timestamp);

            if local_time.date() == current_day.date() {
                sum += value.value.parse::<f64>().expect("String parse error.");
            } else {
                let rec = Record {
                    date: &current_day.date().to_string(),
                    precpitation: &sum,
                };
                wtr.serialize(rec).expect("CSV writer error.");
                sum = 0.0;
                current_day = local_time;
            }
        }

        wtr.flush().expect("Error flushing CSV writer.");
    }
}
