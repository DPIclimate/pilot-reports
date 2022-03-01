// use cargo run -- --nocapture to see println! statements

extern crate dotenv;
use std::env;

mod ubidots;
mod data;
mod datawrapper;
mod utils;

#[test] 
fn weekly_mean() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Organisation key not found");

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    let device_id = "6181b4ca852f0907a66ca289".to_string();

    let variables = ubidots::device::variables::list_variables(&device_id, &token)
        .map_err(|err| println!("Error getting device variables: {}", err))
        .ok().unwrap();

    for variable in &variables.results {
        if config.variables.iter().any(|var| &var.name == &variable.name) {
            let (start, end) = utils::time::one_week();

            let agg = ubidots::device::data::Aggregation {
                variables: vec![variable.id.to_owned()],
                aggregation: "mean".to_string(), 
                join_dataframes: false, 
                start: start,
                end: end,
            };

            let response = agg.aggregate(&token)
                .map_err(|err| println!("Error requesting weekly mean: {}", err))
                .ok().unwrap();

            response.to_csv(&variable.name);
        }
    }
    
}


#[test]
fn load_config() {
    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    for device in &config.devices {
        println!("Device name: {}", device.name);
    }

    // Gets variables as a Vec<String> which can be passed directly into aggregation body
    // This variable now owns the variable names
    let variables = config.list_variable_names();

    for name in &variables {
        println!("Variable name: {}", name);
    }
}

#[test]
fn match_device_to_variables() {
    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Organisation key not found");

    // Get all devcies from Ubidots under specific org
    let all_devices = ubidots::devices::get_all_devices(&token)
        .map_err(|err| println!("Error getting devices list: {}", err))
        .ok().unwrap();

    // Get data from config file
    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    // Check if configured devices are within the requested devices
    for device in &all_devices.results {
        if config.devices.iter().any(|dev| &dev.name == &device.name) {
            println!("Device name: {}", device.name);

            // List variables of device
            let variables = ubidots::device::variables::list_variables(&device.id, &token)
                .map_err(|err| println!("Error getting device variables: {}", err))
                .ok().unwrap();

            // Check if variables are contained within the requested variables (config.json)
            for variable in &variables.results {
                if config.variables.iter().any(|var| &var.name == &variable.name) {
                    println!("Variable name: {}\t Label: {}", variable.name, variable.id)
                }
            }
        }
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
fn create_csv_files() {
    data::files::overwrite_output_csvs();
}
