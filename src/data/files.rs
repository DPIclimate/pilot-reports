use std::fs::OpenOptions;
use crate::utils;

pub fn create_output_csv_files() {
    // Uses the settings in config.json to create .csv files with appropriate columns
    // This need to be run at the start of a new query to clear previous data

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    let path = "data/".to_string();

    for file in &config.files {
        let file_path = format!("{}{}", path, file.name);

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

