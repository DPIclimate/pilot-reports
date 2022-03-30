//! Create line chart dataset
use crate::{ubidots, utils};
use log::{error, info};
use serde::Serialize;
use std::fs::OpenOptions;

#[derive(Debug, Default, Serialize)]
pub struct Chart {
    /// Date e.g. 14/03/2022
    pub date: Vec<String>,
    /// Moonlight harvest area
    pub moonlight: Vec<f64>,
    /// Rocky point harvest area
    pub rocky_point: Vec<f64>,
    /// Waterfall harvest area
    pub waterfall: Vec<f64>,
}

impl Chart {
    /// Method for converting a list of variables into an aggregate weekly dataset of values.
    ///
    /// `variable_list` can be taken from cache or from a new request.
    pub fn new(variable_list: &ubidots::device::variables::VariablesList, token: &String) -> Self {
        info!("Creating chart data");

        let mut chart = Chart {
            date: Vec::new(),
            moonlight: Vec::new(),
            rocky_point: Vec::new(),
            waterfall: Vec::new(),
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
        let (start, end) = utils::time::two_weeks();

        let site_names = vec!["Moonlight", "Rocky Point", "Waterfall"];
        let mut index = 0;
        let mut init = true;
        for ha in harvest_area_variables.as_array() {
            let resampled = ubidots::device::data::Resample {
                variables: ha.to_owned(),
                aggregation: "mean".to_string(),
                join_dataframes: true,
                period: "1D".to_string(),
                start: start * 1000,
                end: end * 1000,
            }
            .resample(&token)
            .map_err(|err| {
                error!(
                    "Error requesting resampled fortnightly chart values: {}",
                    err
                )
            })
            .ok()
            .expect("Error unwrapping resampled average for fortnightly chart view.");

            let mut daily_avg: Vec<f64> = Vec::new();
            for day in resampled.results.iter() {
                if init {
                    let ts = match day[0] {
                        Some(t) => t,
                        None => continue,
                    };
                    chart.date.push(
                        utils::time::unix_to_local(&(ts.round() as i64))
                            .format("%d/%m/%Y")
                            .to_string(),
                    );
                }
                let mut sum = 0.0;
                let mut n = 0.0;
                for value in &day[1..] {
                    let val = match value {
                        Some(v) => v,
                        None => continue,
                    };
                    sum += val;
                    n += 1.0;
                }
                daily_avg.push(sum / n);
            }
            init = false;

            match &site_names[index][..] {
                "Moonlight" => chart.moonlight = daily_avg,
                "Rocky Point" => chart.rocky_point = daily_avg,
                "Waterfall" => chart.waterfall = daily_avg,
                _ => error!(
                    "Unknown harvest area found. Append this harvest area before re-running."
                ),
            }
            index += 1;
        }
        chart
    }

    /// Write a `Chart` struct to csv.
    pub fn to_csv(&self, variable_name: &String) {
        let filename = format!("data/fortnightly-{}-chart.csv", variable_name);

        info!(
            "Publishing fortnightly {} data to {}",
            variable_name, filename
        );

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for day in self.as_array() {
            wtr.write_record(day)
                .expect("CSV serialisation error, fortnightly chart");
        }

        wtr.flush().expect("Error flushing writer");
    }

    pub fn as_array(&self) -> Vec<Vec<String>> {
        let mut transposed = Vec::new();
        for (date, (ml, (rp, wf))) in self.date.iter().zip(
            self.moonlight
                .iter()
                .zip(self.rocky_point.iter().zip(self.waterfall.iter())),
        ) {
            let day: Vec<String> = vec![
                date.to_string(),
                ml.to_string(),
                rp.to_string(),
                wf.to_string(),
            ];
            transposed.push(day)
        }

        transposed
    }
}

/// Vector of variables within a harvest area
struct HarvestAreaVariables {
    /// Moonlight harvest area variables
    moonlight: Vec<String>,
    /// Rocky Point harvest area variables
    rocky_point: Vec<String>,
    /// Waterfall harvest area variables
    waterfall: Vec<String>,
}

impl HarvestAreaVariables {
    pub fn as_array(&self) -> [Vec<String>; 3] {
        [
            self.moonlight.to_owned(),
            self.rocky_point.to_owned(),
            self.waterfall.to_owned(),
        ]
    }
}
