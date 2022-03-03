// use cargo run -- --nocapture to see println! statements
extern crate pilot_reports;
pub use pilot_reports::{data, datawrapper, utils};

extern crate dotenv;
use std::env;

#[test]
fn load_config() {
    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    for device in &config.devices {
        println!("Device name: {}", device.name);
    }

    for name in &config.variables {
        println!("Variable name: {}", name);
    }
}

#[test]
fn unix_timestamp_to_local_day(){
    let ts = 1645491182126;
    let local_day = utils::time::unix_to_local(&ts)
        .date()
        .format("%A");
    assert_eq!("Tuesday".to_string(), local_day.to_string());
}

#[test]
#[ignore]
fn create_csv_files() {
    data::files::create_output_csv_files();
}

#[test]
fn datawrapper_upload() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    for file in &config.files {
        let filepath = file.filepath.to_string();
        println!("Filepath: {}", filepath);
        let chart_id = file.chart_id.to_string();
        datawrapper::export::upload_dataset(&filepath, &chart_id, &dw_key);
        datawrapper::export::publish_chart(&chart_id, &dw_key);
        break;
    }
}

