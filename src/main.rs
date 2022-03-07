extern crate dotenv;
use log::{error, info};
use log4rs;
use std::env;

mod data;
mod datawrapper;
mod ubidots;
mod utils;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // Load env variables from .env file
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Organisation key not found");
    let aws_token = env::var("AWS_ORG_KEY").expect("AWS org key not found");
    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");

    // Get config from config.json
    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    // Overwrite existing .csv files
    data::files::create_output_csv_files(&config);

    // Loop through variables list from config.json
    for variable in &config.variables {
        info!("Processing variable: {}", variable);

        // Construct a list of variables that match devices in config.jon
        let variable_list =
            ubidots::device::variables::get_variables_list(&variable, &config, &token);

        // Create fortnightly csv files
        let fortnight_vec = data::fortnightly::parse(&variable_list, &token);
        data::fortnightly::to_csv(&variable, &fortnight_vec);

        // Create weekly csv files
        let weekly = data::weekly::parse(&variable_list, &token);
        weekly.to_csv(&variable);
    }

    // Write datasets to datawrapper
    data::files::all_files_to_datawrapper(&dw_key, &config);
}
