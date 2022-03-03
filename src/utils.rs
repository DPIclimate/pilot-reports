//! Utilities that don't warrant a particular header but are required 

pub mod time {
    extern crate chrono;
    use chrono::prelude::*;

    pub fn unix_to_local(unix_time: &i64) -> DateTime<Local> {
        // Takes a unix time in ms (conveting it to seconds before parsing)
        // Returns the local time 
        let datetime_ts = Utc.timestamp(unix_time / 1000, 0);
        DateTime::<Local>::from(datetime_ts)
    }

    pub fn one_week() -> (i64, i64) {
        // Get the time a week ago and the current time in UNIX timestamp
        let time_now = Utc::now().timestamp() * 1000;
        let last_week = time_now - 604800000;
        (last_week, time_now)
    }

    pub fn last_week() -> (i64, i64) {
        // Get the time between the start of two weeks ago and the start of last week
        let last_week_end = (Utc::now().timestamp() * 1000) - 604800000;
        let last_week_start = last_week_end - 604800000;
        (last_week_start, last_week_end)
    }

    pub fn weekly_column_names() -> Vec<String> {
        let mut unix_time = (Utc::now().timestamp() * 1000) - 604800000;
        let mut col_names: Vec<String> = Vec::new();
        for _ in 0..7 { 
            let local_day = unix_to_local(&unix_time)
                .date()
                .format("%A");
            col_names.push(local_day.to_string());
            unix_time += 86400000;
        }
        col_names
    }
}


pub mod config {

    use std::fs::File;
    use std::io::BufReader;
    use serde::Deserialize;
    use std::error::Error;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Config {
        pub devices: Vec<Device>,
        pub variables: Vec<String>,
        pub files: Vec<FileConfig>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        pub name: String,
        pub location: String,
        #[serde(rename = "harvest_area")]
        pub harvest_area: String,
        #[serde(rename = "buoy_number")]
        pub buoy_number: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FileConfig {
        pub filename: String,
        pub name: String,
        #[serde(rename = "chart_id")]
        pub chart_id: String,
        pub dynamic: bool,
        pub columns: Vec<String>,
    }

    pub fn get_config() -> Result<Config, Box<dyn Error>> {
        // Get the configuration of devices and variables to use for analysis

        print!("Loading config...");

        let file = File::open("config.json")
            .expect("Error, devices.json file not found.");

        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)
            .expect("Error, device.json should be valid json.");

        println!("loaded");

        Ok(config)
    }
}

