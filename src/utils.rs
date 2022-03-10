pub mod time {
    //! Time related utilities for handling date and time ranges
    extern crate chrono;
    use chrono::prelude::*;
    use log::info;

    /// Takes a unix time in ms (conveting it to seconds before parsing)
    pub fn unix_to_local(unix_time: &i64) -> DateTime<Local> {
        let datetime_ts = Utc.timestamp(unix_time / 1000, 0);
        DateTime::<Local>::from(datetime_ts)
    }

    /// Converts current unix time to local day (e.g. Tuesday)
    pub fn unix_to_local_day(unix_time: &i64) -> String {
        let utc_unix = Utc.timestamp(unix_time / 1000, 0);
        let local_time = DateTime::<Local>::from(utc_unix);
        chrono::Local
            .ymd(local_time.year(), local_time.month(), local_time.day())
            .and_hms(0, 0, 0)
            .format("%A")
            .to_string()
    }

    /// Gets a tuple containing the unix time (in ms) from the start of this year
    /// to the current date
    pub fn this_year() -> (i64, i64) {
        let utc_time_now = Utc::now(); // 2022-03-04 03:24:29.457745 UTC
        let ts_now = utc_time_now.timestamp();
        let local_time_now = DateTime::<Local>::from(utc_time_now);
        let start_of_year = chrono::Local
            .ymd(local_time_now.year() - 1, 12, 31)
            .and_hms(0, 0, 0)
            .timestamp();
        (start_of_year * 1000, ts_now * 1000)
    }

    /// Get the time a week ago and the current time in UNIX timestamp
    /// This gets the current UTC time, subracts a week (UNIX ms) then
    /// rounds down to the start of the day (h = 0, m = 0, sec = 0)
    /// This is needed otherwise running the program at random times would
    /// effect the daily average.
    pub fn one_week() -> (i64, i64) {
        let time_now = Utc::now().timestamp(); // 1646364269
        let utc_time_now = Utc::now(); // 2022-03-04 03:24:29.457745 UTC
        let local_time_now = DateTime::<Local>::from(utc_time_now);
        let midnight_today = chrono::Local
            .ymd(
                local_time_now.year(),
                local_time_now.month(),
                local_time_now.day(),
            )
            .and_hms(0, 0, 0)
            .timestamp();

        let last_week = (midnight_today * 1000) - 518400000;

        info!(
            "Last week UNIX ts: {} Current UNIX ts: {}",
            last_week / 1000,
            time_now
        );
        (last_week, time_now * 1000)
    }

    /// Get the time between the start of two weeks ago and the start of last week
    pub fn last_week() -> (i64, i64) {
        let last_week_end = (Utc::now().timestamp() * 1000) - 604800000;
        let last_week_start = last_week_end - 604800000;
        (last_week_start, last_week_end)
    }

    /// Gets a list of column names containing only the day (e.g. Tuesday, Wednesday etc.)
    pub fn weekly_column_names() -> Vec<String> {
        let (last_week, _now) = one_week();
        let mut unix_time = last_week.to_owned();
        let mut col_names: Vec<String> = Vec::new();
        for _ in 0..7 {
            let local_day = unix_to_local(&unix_time).date().format("%A");
            col_names.push(local_day.to_string());
            unix_time += 86400000;
        }
        info!(
            "Column names range from {} to {}",
            col_names[0],
            col_names[col_names.len() - 1]
        );
        col_names
    }
}

pub mod config {
    //! Handles reading from `config.json` to struct

    use log::info;
    use serde::Deserialize;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;

    /// Configuration from `config.json`
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Config {
        /// List of devices
        pub devices: Vec<Device>,
        /// List of variables of interest
        pub variables: Vec<String>,
        /// List of file configurations
        pub files: Vec<FileConfig>,
        /// Water NSW configuration
        pub water_nsw: WaterNsw,
    }

    /// Device information from `config.json`
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        /// Name of the device
        pub name: String,
        /// Location of devices
        pub location: String,
        /// Corresponding harvest area
        #[serde(rename = "harvest_area")]
        pub harvest_area: String,
        /// Buoy number of device
        #[serde(rename = "buoy_number")]
        pub buoy_number: String,
    }

    /// File configuration from `config.json`
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FileConfig {
        /// Path to output directory (data/... reccommended)
        pub filepath: String,
        /// Name of output file without directory
        pub name: String,
        /// Chart id from datawrapper
        #[serde(rename = "chart_id")]
        pub chart_id: String,
        /// Are the column headers generated dynamically
        pub dynamic: bool,
        /// List of columns
        pub columns: Vec<String>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WaterNsw {
        pub sites: Vec<WaterNswSite>,
        pub defaults: WaterNswDefaults,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WaterNswSite {
        pub name: String,
        pub id: i64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WaterNswDefaults {
        pub params: WaterNswParams,
        pub function: String,
        pub version: i64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WaterNswParams {
        pub varfrom: f64,
        pub interval: String,
        pub varto: f64,
        pub datasource: String,
        #[serde(rename = "data_type")]
        pub data_type: String,
        pub multiplier: i64,
    }

    /// Get the configuration of devices and variables to use for analysis
    /// # Example
    /// ```
    /// use crate::utils::config;
    ///
    /// let config = config::get_config()
    ///    .map_err(|err| println!("Error loading config: {}", err))
    ///    .ok()
    ///    .unwrap();
    /// ```
    ///
    pub fn get_config() -> Result<Config, Box<dyn Error>> {
        info!("Loading config from config.json");

        let file = File::open("config.json").expect("Error, devices.json file not found.");
        let reader = BufReader::new(file);
        let config =
            serde_json::from_reader(reader).expect("Error, device.json should be valid json.");

        Ok(config)
    }
}
