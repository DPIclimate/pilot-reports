//! Handles csv creation and writing.
use crate::datawrapper;
use crate::utils;
use log::{error, info};
use std::fs::OpenOptions;

/// Creates new header only csv files for writing data to.
///
/// Uses information from `config.json` to create csv files for each dataset.
/// If a dynamic (bool) column is specified this will need to be configured
/// maunually. **This function should be run at program start to clear existing
/// data.**
/// # Example `config.json` for a dataset
/// ```json
/// ...
/// "files": {
///     "filepath": "data/fortnightly-temperature.csv", // Output file path
///		"name": "fortnightly-temperature", // Output file name (without extension)
///		"chart_id": "FRGYL", // Chart ID from datawrapper.de
///		"dynamic": false, // Dynamically generate columns
///		"columns": [ // Columns to generate
///			"Location",
///			"Last Week",
///			"This Week",
///			"Harvest Area"
///		]
///	}
/// ```
pub fn create_output_csv_files(config: &utils::config::Config) {
    info!("Overwriting csv files with headers from config.json");

    for file in &config.files {
        info!("Creating {}", file.filepath);
        let csv_file = OpenOptions::new()
            .truncate(true) // Overwrites file if exits (back to zero size)
            .write(true)
            .create(true)
            .open(file.filepath.to_owned())
            .unwrap();

        let mut wtr = csv::Writer::from_writer(csv_file);

        if file.dynamic {
            info!("Creating dynamic columns for {}", file.filepath);
            let mut days_of_week = utils::time::weekly_column_names();
            days_of_week.append(&mut file.columns.to_owned());
            wtr.serialize(days_of_week).expect("Writer error");
        } else {
            wtr.serialize(file.columns.to_owned())
                .expect("Writer error");
        }
    }
}

pub fn all_files_to_datawrapper(dw_key: &String, config: &utils::config::Config) {
    // ---- Write csv's to datawrapper ---- //
    for file in &config.files {
        let filepath = file.filepath.to_string();
        let chart_id = file.chart_id.to_string();
        datawrapper::export::upload_dataset(&filepath, &chart_id, &dw_key)
            .map_err(|err| error!("Error uploading data: {}", err))
            .ok();
        datawrapper::export::publish_chart(&chart_id, &dw_key)
            .map_err(|err| error!("Error publishing chart: {}", err))
            .ok();
    }
}
