use crate::utils;
use log::{error, info};
use std::error::Error;
use std::fs::File;
use std::io::{copy, Read};

#[tokio::main]
pub async fn download_image(
    output_filename: &String,
    chart_id: &String,
    datawrapper_key: &String,
) -> Result<(), Box<dyn Error>> {
    info!(
        "Downloading image from datawrapper.de to {}",
        output_filename
    );

    let url = format!("https://api.datawrapper.de/v3/charts/{}/export/png?unit=px&mode=rgb&plain=false&borderWidth=20", chart_id);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(datawrapper_key.as_str())
        .header("accept", "*/*")
        .send()
        .await?
        .bytes()
        .await?;

    let mut dest = File::create(output_filename)?;

    let mut content = response.as_ref();

    copy(&mut content, &mut dest)?;

    Ok(())
}
