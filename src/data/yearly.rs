use crate::{ubidots, utils};
use log::{error, info};
use polars::prelude::*;
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
struct Record<'a> {
    date: &'a String,
    #[serde(rename(serialize = "2020"))]
    precipitation: &'a f64,
}

pub fn year_to_date_precipitation_to_csv(aws_token: &String) {
    info!("Getting yearly precipitation from Ubidots.");

    let file_path = String::from("data/yearly-precipitation.csv");

    // Variable represents total daily rainfall
    let variables = vec!["61f74ccff6e837004e0691f4".to_string()];
    let raw_series = ubidots::device::aws::RawSeries::new(&variables, utils::time::this_year());

    let precipitation = raw_series
        .get_year_to_date(&aws_token)
        .map_err(|err| error!("Error getting precipitation data: {}", err))
        .ok()
        .unwrap();

    let mut wtr = csv::Writer::from_path(file_path).expect("Unable to find file to write to.");

    let mut sum = 0.0;
    // Little bit hacky but Ubidots gives data in three nested vectors which needs to be handled in
    // reverse
    for d in (0..&precipitation.results[0].len() - 1).rev() {
        let data = &precipitation.results[0][d];
        let (value, ts) = (data[0], (data[1].round() as i64));
        let local_date = utils::time::unix_to_local(&ts).date().format("%Y-%m-%d");
        let rec = Record {
            date: &local_date.to_string(),
            precipitation: &(value.to_owned() + sum),
        };
        sum += value.to_owned();
        wtr.serialize(rec).expect("CSV writer error");
    }

    wtr.flush().expect("Error flushing writer");
}

pub fn join_precipitation_datasets() {
    info!("Joining precipitation datasets");

    let files = vec![
        "data/2020-precipitation.csv".to_string(),
        "data/2021-precipitation.csv".to_string(),
        "data/yearly-precipitation.csv".to_string(),
    ];

    let mut df = DataFrame::default();

    let mut init = true;
    for file in &files {
        let tmp_df = CsvReader::from_path(file)
            .expect("Unable to open precipitation file")
            .infer_schema(None)
            .has_header(true)
            .finish()
            .unwrap();
        if init {
            df = tmp_df.clone();
            init = false;
        } else {
            df = df
                .join(&tmp_df, ["date"], ["date"], JoinType::Outer, None)
                .expect("Unable to join dataframes");
        }
    }

    let mut output_file =
        File::create("data/combined-precipitation.csv").expect("Unable to create combined csv");

    CsvWriter::new(&mut output_file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(&mut df)
        .unwrap();
}
