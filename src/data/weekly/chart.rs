//! Create line chart dataset
use crate::{ubidots, utils};
use log::{error, info};
use std::error::Error;
use std::fs::OpenOptions;

pub struct Chart {
    /// Day of the week e.g. Tuesday
    pub day: Vec<String>,
    /// Waterfall harvest area
    pub waterfall: Vec<f64>,
    /// Moonlight harvest area
    pub moonlight: Vec<f64>,
    /// Rocky point harvest area
    pub rocky_point: Vec<f64>,
}

impl Chart {
    /// Method for converting a list of variables into an aggregate weekly dataset of values.
    ///
    /// `variable_list` can be taken from cache or from a new request.
    pub fn new(variable_list: &ubidots::device::variables::VariablesList, token: &String) -> Self {
        info!("Creating chart data");

        let mut chart = Chart {
            day: Vec::new(),
            waterfall: Vec::new(),
            moonlight: Vec::new(),
            rocky_point: Vec::new(),
        };

        let mut harvest_area_variables = HarvestAreaVariables {
            waterfall: Vec::new(),
            moonlight: Vec::new(),
            rocky_point: Vec::new(),
        };

        // Combine like harvest areas together into a vector that can be passed to ubidots
        for (var_id, ha) in variable_list
            .ids
            .iter()
            .zip(variable_list.harvest_area.iter())
        {
            // Convert String to &str
            match &ha[..] {
                "Moonlight" => harvest_area_variables.moonlight.push(var_id.to_owned()),
                "Rocky Point" => harvest_area_variables.rocky_point.push(var_id.to_owned()),
                "Waterfall" => harvest_area_variables.waterfall.push(var_id.to_owned()),
                _ => error!(
                    "Unknown harvest area found. Append this harvest area before re-running."
                ),
            }
        }
        let (week_start, _week_end) = utils::time::one_week();
        let mut offset = 0;
        for _ in 0..7 {
            let (start, end) = (week_start + offset, week_start + offset + 86400000);

            let local_day = utils::time::unix_to_local_day(&start); // Plain text day e.g. Sunday
            chart.day.push(local_day);

            // Formulate the JSON body for each request and get data
            let moonlight = aggregate_harvest_area_daily(
                &harvest_area_variables.moonlight,
                &token,
                &(start, end),
            )
            .unwrap();

            let ml_value = match moonlight.results[0].value {
                Some(v) => v,
                None => 0.0,
            };
            chart.moonlight.push(ml_value);

            let rocky_point = aggregate_harvest_area_daily(
                &harvest_area_variables.rocky_point,
                &token,
                &(start, end),
            )
            .unwrap();

            let rp_value = match rocky_point.results[0].value {
                Some(v) => v,
                None => 0.0,
            };
            chart.rocky_point.push(rp_value);

            let waterfall = aggregate_harvest_area_daily(
                &harvest_area_variables.waterfall,
                &token,
                &(start, end),
            )
            .unwrap();

            let wf_value = match waterfall.results[0].value {
                Some(v) => v,
                None => 0.0,
            };
            chart.waterfall.push(wf_value);

            offset += 86400000;
        }

        chart
    }

    /// Write a `Chart` struct to csv.
    pub fn to_csv(&self, variable_name: &String) {
        let filename = format!("data/weekly-{}-chart.csv", variable_name);

        info!("Publishing weekly {} data to {}", variable_name, filename);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for i in 0..7 {
            wtr.write_record([
                self.day[i].to_owned(),
                self.moonlight[i].to_string(),
                self.rocky_point[i].to_string(),
                self.waterfall[i].to_string(),
            ])
            .expect("Error creating weekly chart csv.");
        }

        wtr.flush().expect("Error flushing writer");
    }
}

/// Vector of variables within a harvest area
struct HarvestAreaVariables {
    /// Waterfall harvest area variables
    waterfall: Vec<String>,
    /// Moonlight harvest area variables
    moonlight: Vec<String>,
    /// Rocky Point harvest area variables
    rocky_point: Vec<String>,
}

/// Method to aggregate data from harest areas on a daily basis.
fn aggregate_harvest_area_daily(
    variables: &Vec<String>,
    token: &String,
    (start, end): &(i64, i64),
) -> Result<ubidots::device::data::Response, Box<dyn Error>> {
    let aggregation = ubidots::device::data::Aggregation {
        variables: variables.to_vec(),
        aggregation: "mean".to_string(),
        join_dataframes: true, // Returns only one value
        start: start.to_owned(),
        end: end.to_owned(),
    };

    let response = aggregation
        .aggregate(&token)
        .map_err(|err| error!("Error requesting weekly mean: {}", err))
        .ok()
        .unwrap();

    Ok(response)
}
