extern crate dotenv;
use log::{error, info};
use log4rs;
use std::env;

mod cli;
mod data;
mod datawrapper;
mod ubidots;
mod utils;
mod waternsw;

fn main() {
    let cli_config = cli::Config::new();

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
        let variable_list: ubidots::device::variables::VariablesList;
        if cli_config.use_cache {
            variable_list = ubidots::device::variables::VariablesList::new_from_cache(&variable);
        } else {
            variable_list =
                ubidots::device::variables::VariablesList::new(&variable, &config, &token);
            variable_list.cache(&variable);
        }

        // Create fortnightly csv files
        let fortnightly_range_plot =
            data::fortnightly::rangeplot::RangePlot::new(&variable_list, &token);
        fortnightly_range_plot.to_csv(&variable);

        // Create weekly table csv files
        let weekly_table = data::weekly::table::Table::new(&variable_list, &token);
        weekly_table.to_csv(&variable);

        let weekly_chart = data::weekly::chart::Chart::new(&variable_list, &token);
        weekly_chart.to_csv(&variable);
    }

    // Weekly bar chart of precipitation
    data::weekly::bar::weekly_precipitation_to_csv(&aws_token);

    // Make year to data precipitation datasets
    data::yearly::year_to_date_precipitation_to_csv(&aws_token);
    data::yearly::join_precipitation_datasets();

    // Write datasets to datawrapper
    datawrapper::export::all_files_to_datawrapper(&dw_key, &config);
}
