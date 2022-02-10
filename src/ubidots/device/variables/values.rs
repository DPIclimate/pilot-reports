//! Get values for a variable
use serde::Deserialize;
use std::error::Error;
use crate::ubidots::device;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub task: PostId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostId {
    pub id: String,
}

#[tokio::main]
pub async fn get(id: &device::Identifier, token: &String, 
    email: &String, (start, end): &(i64, i64)) -> Result<Status, Box<dyn Error>> {

    let url = match id {
        device::Identifier::ApiLabel(l) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/_/values/export/?email={}&startDate={}&endDate={}&timezone=Australia/Sydney", l, email, start, end)
        },
        device::Identifier::Id(i) => {
            format!("https://industrial.api.ubidots.com/api/v2.0/devices/~{}/_/values/export/?email={}&startDate={}&endDate={}&timezone=Australia/Sydney", i, email, start, end)
        }
    };
    
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("X-Auth-Token", token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Status>()
        .await?;

    Ok(res)
}

