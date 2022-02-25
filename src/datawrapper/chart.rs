use serde::Serialize;
use crate::ubidots;
use crate::utils;

#[derive(Serialize)]
struct Record<'a> {
    date: &'a String,
    precipitation: &'a f64,
}

pub fn json_to_csv(aws: &ubidots::device::aws::WeatherStation) {

    let file_path = String::from("data/precipitation/precipitation.csv");
    
    let mut wtr = csv::Writer::from_path(file_path)
        .expect("Unable to find file to write to.");

    for precip in &aws.results {
        let local_date = utils::time::unix_to_local(&precip.timestamp)
            .date()
            .format("%Y-%m-%d");
        let rec = Record {
            date: &local_date.to_string(), 
            precipitation: &precip.value
        };
        wtr.serialize(rec).expect("CSV writer error");

    }
    wtr.flush().expect("Error flushing writer");
}

