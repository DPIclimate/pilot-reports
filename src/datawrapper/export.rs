//! Process to export csv to datawrapper is as follows
//! 1. Create a chart using their online interface <https://www.datawrapper.de/>
//! 2. Upload formatted data (as a .csv) to their PUT endpoint
//! 3. Set the charts properties (optional as the first step allows you to create a template)
//! 4. Publish the chart via a POST request (this updates the charts url with the latest data)

use log::info;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// Upload dataset (csv file) to datawrapper chart.
///
/// Requires the chart to exist on datawrapper's server and a valid API key.
/// Will panic if file cannot be found or cannot be opened.
#[tokio::main]
pub async fn upload_dataset(
    file_path: &String,
    chart_id: &String,
    datawrapper_key: &String,
) -> Result<(), Box<dyn Error>> {
    info!("Exporting {} to datawrapper.de...", file_path);

    // Read in the file as a raw string
    let body = match File::open(file_path) {
        Ok(mut file) => {
            let mut b = String::new();
            file.read_to_string(&mut b).expect("Error reading file.");
            b
        }
        Err(e) => {
            panic!("Error opening file {}: {}", file_path, e);
        }
    };

    let url = format!("https://api.datawrapper.de/v3/charts/{}/data", chart_id);

    let client = reqwest::Client::new();
    client
        .put(url)
        .bearer_auth(datawrapper_key.as_str())
        .header("Content-Type", "text/csv")
        .body(body)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn publish_chart(
    chart_id: &String,
    datawrapper_key: &String,
) -> Result<(), Box<dyn Error>> {
    // Once a new dataset has been PUT to datawrappers endpoint this method re-runs
    // the publishing of the chart.

    info!(
        "Publishing chart at https://datawrapper.dwcdn.net/{}/",
        chart_id
    );

    let url = format!("https://api.datawrapper.de/v3/charts/{}/publish", chart_id);

    let client = reqwest::Client::new();
    client
        .post(url)
        .bearer_auth(datawrapper_key.as_str())
        .send()
        .await?;

    Ok(())
}
