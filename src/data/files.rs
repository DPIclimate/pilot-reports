use std::fs::OpenOptions;

pub fn overwrite_output_csvs() {
    let filenames = vec![
        "fortnightly-temperature.csv".to_string(),
        "fortnightly-salinity.csv".to_string(),
        "weekly-salinity.csv".to_string(),
        "weekly-precipitation.csv".to_string(),
    ];

    let path = "data/".to_string();

    for filename in &filenames {
        let file_path = format!("{}{}", path, filename);

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        wtr.serialize(("Day", "Value")).expect("Writer error");
    }
}

