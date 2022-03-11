//! Create table dataset
use crate::{ubidots, utils};
use log::{error, info};
use std::fs::OpenOptions;

/// Matches weekly columns as defined in `config.json`.
#[derive(Debug, Clone)]
pub struct Table {
    /// Location of the device (i.e. its common name)
    pub location: Vec<String>,
    /// A vector of vector of values from a variable
    pub daily_value: Vec<Vec<f64>>,
    /// The name of the devices corresponding harvest area
    pub harvest_area: Vec<String>,
}

/// Handles weekly datasets
impl Table {
    /// Method for converting a list of variables into an aggregate weekly dataset of values.
    ///
    /// `variable_list` can be taken from cache or from a new request.
    pub fn new(variable_list: &ubidots::device::variables::VariablesList, token: &String) -> Table {
        // ---- Table Summary ---- //
        let mut weekly = Table {
            location: Vec::new(),
            daily_value: Vec::new(),
            harvest_area: Vec::new(),
        };

        // Get device location  and harvest area
        for (cd, ha) in variable_list
            .corresponding_device
            .iter()
            .zip(variable_list.harvest_area.iter())
        {
            weekly.location.push(cd.to_string());
            weekly.harvest_area.push(ha.to_string());
        }

        let (week_start, _week_end) = utils::time::one_week();

        let mut offset = 0;
        let mut day_offset = 86400000;
        // Seven days
        for _ in 0..7 {
            let daily_agg = ubidots::device::data::Aggregation {
                variables: variable_list.ids.to_owned(),
                aggregation: "mean".to_string(),
                join_dataframes: false,
                start: week_start + offset,
                end: week_start + day_offset,
            };

            let daily = daily_agg
                .aggregate(&token)
                .map_err(|err| error!("Error requesting weekly mean: {}", err))
                .ok()
                .unwrap();

            let mut day_vec: Vec<f64> = Vec::new();
            for day in daily.results.iter() {
                day_vec.push(day.value.to_owned());
            }

            weekly.daily_value.push(day_vec.to_owned());

            offset += 86400000;
            day_offset += 86400000;
        }
        weekly
    }

    /// Transfoms a weekly struct into a csv file.
    ///
    /// This function also does some basic cleaning by replacing negitive
    /// and extreme values will empty strings. These will be ignored by
    /// datawrapper.de
    pub fn to_csv(&self, variable_name: &String) {
        let filename = format!("data/weekly-{}-table.csv", variable_name);

        info!("Publishing weekly {} data to {}", variable_name, filename);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for (i, (loc, ha)) in self
            .location
            .iter()
            .zip(self.harvest_area.iter())
            .enumerate()
        {
            let mut day_transpose: Vec<String> = Vec::new();
            for day in self.daily_value.iter() {
                // Zero values or values above 40 represent un-reponsive devices
                // These should be represented as null in the csv
                if day[i as usize] > 0.0 && day[i as usize] < 40.0 {
                    day_transpose.push(day[i as usize].to_string());
                } else {
                    let null = "".to_string();
                    day_transpose.push(null.to_owned());
                }
            }
            day_transpose.push(loc.to_string());
            day_transpose.push(ha.to_string());
            wtr.serialize([day_transpose]).expect("Serialization error");
        }
        wtr.flush().expect("Error flushing writer");
    }
}
