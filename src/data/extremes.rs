use crate::{ubidots, utils};
use log::{error, info};
use serde::Serialize;
use std::fs::OpenOptions;

#[derive(Debug, Default, Serialize)]
pub struct Extremes {
    pub rows: Vec<Row>,
}

#[derive(Debug, Default, Serialize)]
pub struct Row {
    /// Harvest area
    pub site: String,
    /// Average minimum for site
    pub min: f64,
    /// Average maximum for site
    pub max: f64,
}

/// Vector of variables within a harvest area
#[derive(Debug)]
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

impl Extremes {
    pub fn new(variable_list: &ubidots::device::variables::VariablesList, token: &String) -> Self {
        let mut extremes = Extremes { rows: Vec::new() };

        let mut harvest_area_variables = HarvestAreaVariables {
            waterfall: Vec::new(),
            moonlight: Vec::new(),
            rocky_point: Vec::new(),
        };

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

        let (start, end) = utils::time::one_week();

        let site_names = vec!["Moonlight", "Rocky Point", "Waterfall"];
        let mut index = 0;
        for ha in harvest_area_variables.as_array() {
            let weekly_min_agg = ubidots::device::data::Aggregation {
                variables: ha.to_owned(),
                aggregation: "min".to_string(),
                join_dataframes: false,
                start: start,
                end: end,
            };

            let weekly_min = match weekly_min_agg.aggregate(&token) {
                Ok(r) => r,
                Err(e) => panic!("Error getting aggregate min: {}", e),
            };

            let mut abs_min = 0.0;
            let mut init = true;
            for min in weekly_min.results.iter() {
                let min_value = match min.value {
                    Some(v) => v,
                    None => continue,
                };

                if init {
                    if min_value > 0.0 {
                        abs_min = min_value;
                        init = false;
                        continue;
                    }
                }
                if min_value < abs_min && min_value >= 0.0 {
                    abs_min = min_value;
                }
            }

            let weekly_max = ubidots::device::data::Aggregation {
                variables: ha.to_owned(),
                aggregation: "max".to_string(),
                join_dataframes: false,
                start: start,
                end: end,
            }
            .aggregate(&token)
            .map_err(|err| error!("Error requesting aggregate max value: {}", err))
            .ok()
            .expect("Error unwrapping aggregate max");

            let mut abs_max = 0.0;
            let mut location = "".to_string();
            let mut init = true;
            for max in weekly_max.results.iter() {
                let max_value = match max.value {
                    Some(v) => v,
                    None => continue,
                };
                if init {
                    if max_value < 45.0 {
                        abs_max = max_value;
                        location = site_names[index].to_string();
                        init = false;
                        continue;
                    }
                }
                if max_value > abs_max && max_value < 45.0 {
                    abs_max = max_value;
                    location = site_names[index].to_string();
                }
            }

            extremes.rows.push(Row {
                site: location.to_string(),
                min: abs_min,
                max: abs_max,
            });

            index += 1;
        }
        extremes
    }

    pub fn to_csv(&self, variable_name: &String) {
        let filename = format!("data/weekly-{}-extremes.csv", variable_name);

        info!("Publishing weekly {} data to {}", variable_name, filename);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        for row in self.rows.iter() {
            wtr.write_record([
                row.site.to_owned(),
                row.min.to_string(),
                row.max.to_string(),
            ])
            .expect("Error creating weekly chart csv.");
        }

        wtr.flush().expect("Error flushing writer");
    }
}
