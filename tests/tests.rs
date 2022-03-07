// use cargo run -- --nocapture to see println! statements
extern crate pilot_reports;
pub use pilot_reports::{data, datawrapper, ubidots, utils};

extern crate dotenv;
use std::env;

#[test]
#[ignore]
fn load_config() {
    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for device in &config.devices {
        println!("Device name: {}", device.name);
    }

    for name in &config.variables {
        println!("Variable name: {}", name);
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
fn create_csv_files() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
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
        .map_err(|err| println!("Error loading config: {}", err))
        .ok()
        .unwrap();

    for file in &config.files {
        let filepath = file.filepath.to_string();
        println!("Filepath: {}", filepath);
        let chart_id = file.chart_id.to_string();
        datawrapper::export::upload_dataset(&filepath, &chart_id, &dw_key)
            .map_err(|err| println!("Error: {}", err))
            .ok();
        datawrapper::export::publish_chart(&chart_id, &dw_key)
            .map_err(|err| println!("Error: {}", err))
            .ok();
        break;
    }
}

#[test]
#[ignore]
fn aws_to_datawrapper() {
    dotenv::dotenv().expect("Failed to read .env file.");

    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");
    let aws_token = env::var("AWS_ORG_KEY").expect("AWS org key not found");
    let precip_id = env::var("PRECIP_ID").expect("Unable to find precip chart id");

    let aws = ubidots::device::aws::weekly_precipitation(&aws_token)
        .map_err(|err| println!("{}", err))
        .ok()
        .expect("Precipitation parse error.");

    ubidots::device::aws::json_to_csv(&aws);

    let aws_filepath = "data/weekly-precipitation.csv".to_string();
    datawrapper::export::upload_dataset(&aws_filepath, &precip_id, &dw_key)
        .map_err(|err| println!("Error uploading data: {}", err))
        .ok();

    datawrapper::export::publish_chart(&precip_id, &dw_key)
        .map_err(|err| println!("Error publishing chart: {}", err))
        .ok();
}
