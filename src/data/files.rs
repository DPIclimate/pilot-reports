use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use crate::utils;

pub fn create_output_csv_files() {
    // Uses the settings in config.json to create .csv files with appropriate columns
    // This need to be run at the start of a new query to clear previous data

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    let path = "data/".to_string();

    for file in &config.files {
        let file_path = format!("{}{}", path, file.filename);

        let csv_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(csv_file);

        // Dynamic file means that the files headers need to be generated with some function hence
        // the use of weekly column names in this instance
        if file.dynamic {
            let mut days_of_week = utils::time::weekly_column_names();
            days_of_week.append(&mut file.columns.to_owned());
            wtr.serialize(days_of_week).expect("Writer error");
        } else {
            wtr.serialize(file.columns.to_owned()).expect("Writer error");
        }
    }
}

// Setup the structs that corresponds to the csv files (don't know how to do this dynamically)
#[derive(Serialize)]
pub struct Fortnightly {
    location: String,
    last_week: String,
    this_week: String,
}

pub fn fortnightly_to_csv(variable_name: &String, fortnightly: &Vec<Fortnightly>) {
    // Take a vector of fortnightly values and put them into a csv

    let file = format!("data/{}-test.csv", variable_name);

    let mut wtr = csv::Writer::from_path(file)
        .expect("Unable to write to request file path / filename");

    for row in fortnightly.iter() {
        wtr.serialize(row).expect("Unable to write to CSV");
    }

    wtr.flush().expect("Error flushing writer");
}


// Days are used here as their name is dynamic
pub struct Weekly {
    day1: f64, 
    day2: f64, 
    day3: f64, 
    day4: f64, 
    day5: f64, 
    day6: f64, 
    day7: f64, 
    location: String,
    harvest_area: String,
}

