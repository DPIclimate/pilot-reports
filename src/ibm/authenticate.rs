use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessHandler {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "id_token")]
    pub id_token: Option<String>,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    pub scope: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
}

impl AccessHandler {
    #[tokio::main]
    pub async fn new(ibm_key: &String) -> Result<Self, Box<dyn Error>> {
        info!("Requesting new access token from IBM");

        let url: String = String::from("https://auth-b2b-twc.ibm.com/connect/token");

        let mut data = HashMap::new();
        data.insert(String::from("client_id"), String::from("ibm-pairs"));
        data.insert(String::from("grant_type"), String::from("apikey"));
        data.insert(String::from("apikey"), ibm_key.to_string());

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .form(&data) // Automatically adds applicaiton/x-www-form-urlencoded header
            .send()
            .await?
            .json::<AccessHandler>()
            .await?;

        Ok(response)
    }
}
