//use polars::prelude::{CsvReader, SerReader, DataFrame};
use polars::prelude::*;
use std::fs::File;
use std::error::Error;


pub fn to_summary() -> Result<DataFrame>{
    // Transform single csv file (from a device) into a summary that will
    // work with datawrapper (https://www.datawrapper.de/)

    // Define path (TODO make dynamic)
    let path = "data/clyde-salinity20b.csv";

    CsvReader::from_path(&path).expect("Unable to open file at path")
        .infer_schema(None)
        .has_header(true)
        .finish()
}

pub fn get_column_max_f64(df: &DataFrame, column: &String) -> f64 {
    // Get max value of a column containing f64 values
    match df.column(column).unwrap().max::<f64>() {
        Some(v) => v,
        None => 0.0
    }
}

