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

    pub fn fortnight() -> (i64, i64) {
        // Get the time a fortnight ago and the current time in UNIX timestamp
        let time_now = Utc::now().timestamp() * 1000;
        let last_week = time_now - 1209600000;
        (last_week, time_now)
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
        pub variables: Vec<Variable>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        pub name: String,
        pub location: String,
        #[serde(rename = "buoy_number")]
        pub buoy_number: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Variable {
        pub name: String,
    }

    impl Config {
        pub fn list_variable_names(&self) -> Vec<String> {
            // Returns a vector of strings containing variable names
            let mut variables = Vec::new();
            for variable in &self.variables {
                variables.push(variable.name.to_owned());
            }
            variables
        }
    }

    pub fn get_config() -> Result<Config, Box<dyn Error>> {
        // Get the configuration of devices and variables to use for analysis

        let file = File::open("config.json")
            .expect("Error, devices.json file not found.");

        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)
            .expect("Error, device.json should be valid json.");

        Ok(config)
    }
}

