use crate::{ubidots, utils};
use log::{error, info};
use serde::Serialize;

#[derive(Serialize)]
struct Record<'a> {
    date: &'a String,
    precipitation: &'a f64,
}

pub fn weekly_precipitation_to_csv(aws_token: &String) {
    info!("Getting yearly precipitation from Ubidots.");

    let file_path = String::from("data/weekly-precipitation.csv");

    // Variable represents total daily rainfall
    let variables = vec!["61f74ccff6e837004e0691f4".to_string()];
    let raw_series = ubidots::device::aws::RawSeries::new(&variables, utils::time::one_week());

    let precipitation = raw_series
        .get_precipitation(&aws_token)
        .map_err(|err| error!("Error getting precipitation data: {}", err))
        .ok()
        .unwrap();

    let mut wtr = csv::Writer::from_path(file_path).expect("Unable to find file to write to.");

    // Little bit hacky but Ubidots gives data in three nested vectors which needs to be handled in
    // reverse
    for d in (0..&precipitation.results[0].len() - 1).rev() {
        let data = &precipitation.results[0][d];
        let (value, ts) = (data[0], (data[1].round() as i64));
        let local_date = utils::time::unix_to_local(&ts).date().format("%m/%d/%Y");
        let rec = Record {
            date: &local_date.to_string(),
            precipitation: &value.to_owned(),
        };
        wtr.serialize(rec).expect("CSV writer error");
    }

    wtr.flush().expect("Error flushing writer");
}
