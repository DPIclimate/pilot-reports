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

