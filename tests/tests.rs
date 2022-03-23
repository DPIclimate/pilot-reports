// use cargo run -- --nocapture to see println! statements
extern crate pilot_reports;
pub use pilot_reports::{data, datawrapper, ubidots, utils, waternsw};

extern crate dotenv;
use log::{error, info};
use std::env;

#[test]
#[ignore]
fn weekly_extremes() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Org key not found");
    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    // -- Overwrite csv files -- //
    data::files::create_output_csv_files(&config);

    let variable = String::from("salinity");
    let variable_list = ubidots::device::variables::VariablesList::new(&variable, &config, &token);
    data::extremes::Extremes::new(&variable_list, &token).to_csv(&variable);

    let variable = String::from("temperature");
    let variable_list = ubidots::device::variables::VariablesList::new(&variable, &config, &token);
    data::extremes::Extremes::new(&variable_list, &token).to_csv(&variable);
}

#[test]
fn water_nsw() {
    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    // -- Overwrite csv files -- //
    data::files::create_output_csv_files(&config);

    // Fortnightly dataset
    let time_range = String::from("fortnightly");
    waternsw::flow::DischargeRate::generate(&time_range, &config);

    // Yearly dataset
    let time_range = String::from("yearly");
    waternsw::flow::DischargeRate::generate(&time_range, &config);

    // Join datasets
    data::yearly::join_flow_datasets();
}

#[test]
#[ignore]
fn download_plots() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");

    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for file in &config.files {
        let filename = format!("report/imgs/{}.png", file.name);
        datawrapper::download::download_image(&filename, &file.chart_id, &dw_key)
            .map_err(|err| error!("Error downloading image: {}", err))
            .ok()
            .unwrap();
    }
}

#[test]
#[ignore]
fn cache_variables() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Org key not found");

    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for variable in &config.variables {
        // Construct a list of variables that match devices in config.jon
        let variable_list =
            ubidots::device::variables::VariablesList::new(&variable, &config, &token);
        variable_list.cache(&variable);
    }
}

#[test]
#[ignore]
fn create_weekly_chart() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Org key not found");
    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    // -- Overwrite csv files -- //
    data::files::create_output_csv_files(&config);

    // Construct a list of variables that match devices in config.jon
    let variable = String::from("salinity");
    let variable_list = ubidots::device::variables::VariablesList::new(&variable, &config, &token);
    let chart = data::weekly::chart::Chart::new(&variable_list, &token);
    chart.to_csv(&variable);
}

#[test]
#[ignore]
fn create_dataframe() {
    data::yearly::join_precipitation_datasets();
}

#[test]
#[ignore]
fn precipitation_to_csvs() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let aws_token = env::var("AWS_ORG_KEY").expect("AWS org key not found");
    data::weekly::bar::weekly_precipitation_to_csv(&aws_token);
    data::yearly::year_to_date_precipitation_to_csv(&aws_token);
    data::yearly::join_precipitation_datasets();
}

#[test]
#[ignore]
fn load_config() {
    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for device in &config.devices {
        info!("Device name: {}", device.name);
    }

    for name in &config.variables {
        info!("Variable name: {}", name);
    }
}

#[test]
#[ignore]
fn weekly_timestamp() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    utils::time::one_week();
    utils::time::weekly_column_names();
}

#[test]
#[ignore]
fn unix_timestamp_to_local_day() {
    let ts = 1645491182126;
    let local_day = utils::time::unix_to_local(&ts).date().format("%A");
    assert_eq!("Tuesday".to_string(), local_day.to_string());
}

#[test]
#[ignore]
fn create_csv_files() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    data::files::create_output_csv_files(&config);
}

#[test]
#[ignore]
fn datawrapper_upload() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");

    let config = utils::config::get_config()
        .map_err(|err| error!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for file in &config.files {
        let filepath = file.filepath.to_string();
        info!("Filepath: {}", filepath);
        let chart_id = file.chart_id.to_string();
        datawrapper::export::upload_dataset(&filepath, &chart_id, &dw_key)
            .map_err(|err| error!("Error: {}", err))
            .ok();
        datawrapper::export::publish_chart(&chart_id, &dw_key)
            .map_err(|err| error!("Error: {}", err))
            .ok();
        break;
    }
}
