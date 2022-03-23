use crate::utils;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::OpenOptions;

// ---- Start of request body ---- //
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    params: Params,
    function: String,
    version: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Params {
    #[serde(rename = "site_list")]
    site_list: i64,
    #[serde(rename = "start_time")]
    start_time: i64,
    varfrom: f64,
    interval: String,
    varto: f64,
    datasource: String,
    #[serde(rename = "end_time")]
    end_time: i64,
    #[serde(rename = "data_type")]
    data_type: String,
    multiplier: i64,
}
// ---- End of request body ---- //

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DischargeRate {
    #[serde(rename = "error_num")]
    pub error_num: i64,
    #[serde(rename = "return")]
    pub return_field: Return,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub traces: Vec<Trace>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    #[serde(rename = "error_num")]
    pub error_num: i64,
    pub compressed: String,
    #[serde(rename = "site_details")]
    pub site_details: SiteDetails,
    pub trace: Vec<Trace2>,
    pub site: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteDetails {
    pub timezone: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    pub longitude: String,
    pub name: String,
    pub latitude: String,
    #[serde(rename = "org_name")]
    pub org_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trace2 {
    pub v: String,
    pub t: f64,
    pub q: f64,
}

impl DischargeRate {
    #[tokio::main]
    pub async fn new(
        (start, end): &(String, String),
        config: &utils::config::Config,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        info!("Getting discharge rate data from sites within config.json");

        let mut discharge_rate_vec: Vec<DischargeRate> = Vec::new();

        for site in &config.water_nsw.sites {
            let params = Params {
                site_list: site.id,
                start_time: start.parse::<i64>().unwrap(),
                varfrom: config.water_nsw.defaults.params.varfrom.to_owned(),
                interval: config.water_nsw.defaults.params.interval.to_owned(),
                varto: config.water_nsw.defaults.params.varto.to_owned(),
                datasource: config.water_nsw.defaults.params.datasource.to_owned(),
                end_time: end.parse::<i64>().unwrap(),
                data_type: config.water_nsw.defaults.params.data_type.to_owned(),
                multiplier: config.water_nsw.defaults.params.multiplier.to_owned(),
            };

            let request = Request {
                params: params,
                function: config.water_nsw.defaults.function.to_owned(),
                version: config.water_nsw.defaults.version,
            };

            let req_str = serde_json::to_string(&request)?;

            let url = format!(
                "https://realtimedata.waternsw.com.au/cgi/webservice.exe?{}",
                req_str
            );

            // Requrired by WaterNSW for some reason
            static USER_AGENT: &str =
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

            let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
            let res = client
                .get(url)
                .send()
                .await?
                .json::<DischargeRate>()
                .await?;

            discharge_rate_vec.push(res);
        }

        Ok(discharge_rate_vec)
    }

    pub fn to_csv(&self, time_range: &String, filename: &String) -> Result<(), Box<dyn Error>> {
        info!("Publishing discharge rate data to {}", filename);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();

        let mut wtr = csv::Writer::from_writer(file);

        let mut sum = 0.0;
        for trace in &self.return_field.traces[0].trace {
            let date_str = trace.t.to_string(); // e.g. 20220302000000
            let date_arr = date_str
                .chars()
                .enumerate()
                .flat_map(|(i, c)| {
                    if i == 4 || i == 6 {
                        Some('/')
                    } else if i == 8 {
                        Some(' ')
                    } else {
                        None
                    }
                    .into_iter()
                    .chain(std::iter::once(c))
                })
                .collect::<String>(); // e.g. "2022/03/02 000000"

            let date: Vec<&str> = date_arr.split(' ').collect(); // e.g. ["2022/03/02", "000000"]
            let flow = match &time_range[..] {
                "fortnightly" => trace.v.to_owned(),
                "yearly" => {
                    let tmp_volume = trace.v.parse::<f64>()?;
                    let volume = sum + tmp_volume;
                    sum += tmp_volume;
                    volume.to_string()
                }
                _ => panic!("Unknown time range specified. Append this range before re-running"),
            };
            wtr.write_record([date[0].to_owned(), flow.to_string(), trace.q.to_string()])?;
        }

        wtr.flush()?;

        Ok(())
    }

    pub fn generate(time_range: &String, config: &utils::config::Config) {
        info!("Generating discharge rate datasets");

        let unix_range = match &time_range[..] {
            "fortnightly" => utils::time::two_weeks(),
            "yearly" => utils::time::this_year(false),
            _ => panic!("Unknown time range specified. Append this range before re-running"),
        };

        // Create a current timestring (e.g. 20190201000000)
        let timestring_range = utils::time::unix_range_to_timestring(&(unix_range));
        match DischargeRate::new(&timestring_range, &config) {
            Ok(v) => {
                for site in &v {
                    let mut filename = format!(
                        "data/{}-{}.csv",
                        time_range, site.return_field.traces[0].site
                    );
                    for s in &config.water_nsw.sites {
                        if s.id.to_string() == site.return_field.traces[0].site {
                            filename = format!("data/{}-{}.csv", time_range, s.name);
                        }
                    }
                    site.to_csv(&time_range, &filename)
                        .map_err(|err| error!("Error writing Water NSW data to csv: {}", err))
                        .ok();
                }
            }
            Err(e) => error!("There was an error: {:?}", e),
        };
    }
}
