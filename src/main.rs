extern crate dotenv;
use std::env;

mod ubidots;
mod datawrapper;
mod utils;
mod data;

fn main() {

    dotenv::dotenv().expect("Failed to read .env file.");
    let token = env::var("ORG_KEY").expect("Organisation key not found");

    let config = utils::config::get_config()
        .map_err(|err| println!("Error loading config: {}", err))
        .ok().unwrap();

    // Overwrite existing .csv files
    data::files::create_output_csv_files(&config);

    // Get all devcies from Ubidots under specific org
    let all_devices = ubidots::devices::get_all_devices(&token)
        .map_err(|err| println!("Error getting devices list: {}", err))
        .ok().unwrap();

    for variable in &config.variables {
        println!("Processing variable: {}", variable);

        let mut variable_list = ubidots::device::variables::VariablesList {
            name: variable.to_string(),
            ids: Vec::new(),
            corresponding_device: Vec::new(),
            harvest_area: Vec::new(),
        };

        for device in &all_devices.results {
            if config.devices.iter().any(|dev| &dev.name == &device.name) {
                // List variables of device
                let all_variables = ubidots::device::variables::list_variables(&device.id, &token)
                    .map_err(|err| println!("Error getting device variables: {}", err))
                    .ok().unwrap();

                // Check if variables are contained within the requested variables (config.json)
                for var in &all_variables.results {
                    if &var.name == variable {
                        let mut location: String = "unknown".to_string();
                        let mut harvest_area: String = "unknown".to_string();
                        for dev in &config.devices {
                            if &dev.name == &device.name {
                                location = dev.location.to_owned();
                                harvest_area = dev.harvest_area.to_owned();
                                break;
                            }
                        }
                        variable_list.add_variable_and_device(&var.id, &location, &harvest_area);
                        break;
                    }
                }
            }
        }

        // ---- Last Week ---- //
        let (start, end) = utils::time::one_week();

        let this_week_agg = ubidots::device::data::Aggregation {
            variables: variable_list.ids.to_owned(),
            aggregation: "mean".to_string(), 
            join_dataframes: false, 
            start: start,
            end: end,
        };

        let this_week = this_week_agg.aggregate(&token)
            .map_err(|err| println!("Error requesting weekly mean: {}", err))
            .ok().unwrap();

        // ---- This Week ---- //
        let (start, end) = utils::time::last_week();

        let last_week_agg = ubidots::device::data::Aggregation {
            variables: variable_list.ids.to_owned(),
            aggregation: "mean".to_string(), 
            join_dataframes: false, 
            start: start,
            end: end,
        };

        let last_week = last_week_agg.aggregate(&token)
            .map_err(|err| println!("Error requesting weekly mean: {}", err))
            .ok().unwrap();

        let mut fortnight_vec: Vec<data::files::Fortnightly> = Vec::new();

        for (lw, (tw, (cd, ha))) in last_week.results.iter().zip(this_week.results.iter()
                .zip(variable_list.corresponding_device.iter()
                    .zip(variable_list.harvest_area.iter()))) {

            let fortnight = data::files::Fortnightly {
                location: cd.to_string(),
                last_week: lw.value,
                this_week: tw.value,
                harvest_area: ha.to_string(),
            };
            fortnight_vec.push(fortnight);
        }

        data::files::fortnightly_to_csv(&variable, &fortnight_vec);

        // ---- Weekly Summary ---- //
        let mut weekly = data::files::Weekly {
            location: Vec::new(),
            daily_value: Vec::new(),
            harvest_area: Vec::new(),
        };

        // Get device location  and harvest area
        for (cd, ha) in variable_list.corresponding_device.iter().zip(variable_list.harvest_area.iter()) {
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

            let daily = daily_agg.aggregate(&token)
                .map_err(|err| println!("Error requesting weekly mean: {}", err))
                .ok().unwrap();

            let mut day_vec: Vec<f64> = Vec::new();
            for day in daily.results.iter() {
                day_vec.push(day.value.to_owned());
            }

            weekly.daily_value.push(day_vec.to_owned());

            offset += 86400000;
            day_offset += 86400000;
        }

       weekly.to_csv(&variable);
    }

    // ---- Write csv's to datawrapper ---- //
    let dw_key = env::var("DW_KEY").expect("Datawrapper key not found");

    for file in &config.files {
        let filepath = file.filepath.to_string();
        println!("Filepath: {}", filepath);
        let chart_id = file.chart_id.to_string();
        datawrapper::export::upload_dataset(&filepath, &chart_id, &dw_key)
            .map_err(|err| println!("Error uploading data: {}", err))
            .ok();
        datawrapper::export::publish_chart(&chart_id, &dw_key)
            .map_err(|err| println!("Error publishing chart: {}", err))
            .ok();
    }

    // ---- Custom push to datawrapper for AWS ---- //
    let aws_token = env::var("AWS_ORG_KEY").expect("AWS org key not found");
    let precip_id = env::var("PRECIP_ID").expect("Unable to find precip chart id");

    let aws = ubidots::device::aws::weekly_precipitation(&aws_token)
        .map_err(|err| println!("{}", err))
        .ok().expect("Precipitation parse error.");

    ubidots::device::aws::json_to_csv(&aws);

    datawrapper::export::publish_chart(&precip_id, &dw_key)
        .map_err(|err| println!("Error publishing chart: {}", err))
        .ok();
}

