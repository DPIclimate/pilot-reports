//! Data methods for transforming raw values into a RangePlot on datawrapper.de. These functions
//! build and tranform data to .csv files
use crate::{ubidots, utils};
use log::{error, info};
use std::fs::OpenOptions;

/// Used to store row data for Range Plots
pub struct RangePlot {
    /// Vector of `RangeValues`
    pub range_values: Vec<RangeValues>,
}

/// Matches columns defined in config.json.
/// These specifiy the order and type of data found in each fortnightly dataset.
/// Contains a single row of data which is the passed to the `RangePlot` struct.
pub struct RangeValues {
    /// Location of the device
    pub location: String,
    /// The average reading last week (two weeks ago)
    pub last_week: f64,
    /// The average reading this week
    pub this_week: f64,
    /// The corresponding harvest area for buoys
    pub harvest_area: String,
}

impl RangePlot {
    /// Method for converting a list of variables into a aggregate fortnightly dataset of values.
    ///
    /// `variable_list` can be taken from cache or from a new request. This method returns a vector
    /// of `RangePlot` structs .
    pub fn new(variable_list: &ubidots::device::variables::VariablesList, token: &String) -> Self {
        // ---- Last Week ---- //
        let (start, end) = utils::time::one_week();

        let this_week_agg = ubidots::device::data::Aggregation {
            variables: variable_list.ids.to_owned(),
            aggregation: "mean".to_string(),
            join_dataframes: false,
            start: start,
            end: end,
        };

        let this_week = this_week_agg
            .aggregate(&token)
            .map_err(|err| error!("Error requesting weekly mean: {}", err))
            .ok()
            .unwrap();

        // ---- This Week ---- //
        let (start, end) = utils::time::last_week();

        let last_week_agg = ubidots::device::data::Aggregation {
            variables: variable_list.ids.to_owned(),
            aggregation: "mean".to_string(),
            join_dataframes: false,
            start: start,
            end: end,
        };

        let last_week = last_week_agg
            .aggregate(&token)
            .map_err(|err| error!("Error requesting weekly mean: {}", err))
            .ok()
            .unwrap();

        let mut range_plot = RangePlot {
            range_values: Vec::new(),
        };

        for (lw, (tw, (cd, ha))) in last_week.results.iter().zip(
            this_week.results.iter().zip(
                variable_list
                    .corresponding_device
                    .iter()
                    .zip(variable_list.harvest_area.iter()),
            ),
        ) {
            let values = RangeValues {
                location: cd.to_string(),
                last_week: lw.value,
                this_week: tw.value,
                harvest_area: ha.to_string(),
            };
            range_plot.range_values.push(values);
        }
        range_plot
    }

    /// Takes a vector of fortnightly data and transforms it into the corresponding
    /// csv file as found in `config.json`.
    ///
    /// The `variable_name` is used to write to the correct csv file. Note this
    /// function differs from `weekly_to_csv` as this function takes a vector of
    /// fortnightly values.
    ///
    /// This function also does some basic data cleaning. This involves replacing
    /// negative values and extreme values with empty strings. This allows them to
    /// be ignored by datawrapper.de
    pub fn to_csv(&self, variable_name: &String) {
        let filename = format!("data/fortnightly-{}.csv", variable_name);

        info!("Publishing weekly {} data {}", variable_name, filename);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for row in self.range_values.iter() {
            // Zero values or values above 40 represent un-reponsive devices
            // These should be represented as null in the csv
            if row.this_week > 0.0
                && row.this_week < 40.0
                && row.last_week > 0.0
                && row.last_week < 40.0
            {
                wtr.write_record([
                    row.location.to_owned(),
                    row.last_week.to_string(),
                    row.this_week.to_string(),
                    row.harvest_area.to_owned(),
                ])
                .expect("Unable to write to CSV");
            } else {
                wtr.write_record([
                    row.location.to_owned(),
                    "".to_string(),
                    "".to_string(),
                    row.harvest_area.to_owned(),
                ])
                .expect("Unable to write to CSV");
            }
        }

        wtr.flush().expect("Error flushing writer");
    }
}
