use std::fs::OpenOptions;
use serde::Serialize;
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
            .truncate(true) // Overwrites file if exits (back to zero size)
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
    pub location: String,
    pub last_week: f64,
    pub this_week: f64,
    pub harvest_area: String, 
}

pub fn fortnightly_to_csv(variable_name: &String, fortnightly: &Vec<Fortnightly>) {
    // Take a vector of fortnightly values and put them into a csv

    let filename = format!("data/fortnightly-{}.csv", variable_name);

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for row in fortnightly.iter() {
        wtr.write_record([row.location.to_owned(),
                        row.last_week.to_string(), 
                        row.this_week.to_string(),
                        row.harvest_area.to_owned()])
            .expect("Unable to write to CSV");
    }

    wtr.flush().expect("Error flushing writer");
}


pub struct Weekly {
    pub location: Vec<String>,
    pub daily_value: Vec<f64>,
    pub harvest_area: Vec<String>,
}

impl Weekly {
    pub fn to_csv(&self) {
        let filename = format!("data/weekly-{}.csv", variable_name);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for (loc, (dv, ha)) in self.location().iter().zip(self.daily_value.iter().zip(harvest_area.iter)) {
            // start here tomorrow
            wtr.write_record([])
        }
        wtr.flush().expect("Error flushing writer");
    }
}

