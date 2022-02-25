extern crate dotenv;
use std::{thread, time};
use std::env;

mod ubidots;
mod gmail;
mod data;
mod datawrapper;
mod utils;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let email = env::var("GMAIL_ADDR").expect("Email address not found");

    let access = gmail::authenticate::run();

    let list_messages = gmail::messages::list(&access, &email)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    gmail::messages::clear_inbox(&access, &email, &list_messages)
        .map_err(|err| println!("{:#?}", err))
        .ok();

    let org_token = env::var("ORG_KEY").expect("Organisation key not found");
    
    // Temp set these a fixed
    let end_date = 1645361999000;
    let start_date = 1644152400000;
    let date_range = (start_date, end_date);

    let devices = ubidots::device::get_all(&org_token)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    // Export all data from an org to an email (defined in .env)
    let res = ubidots::export::data_to_email(&org_token, &email, &devices, &date_range)
        .map_err(|err| println!("Email error: {}", err))
        .ok().unwrap();

    // Wait until all messages are accounted for in the gmail inbox
    let mut n_retries = 0;
    loop {
        n_retries += 1;
        let list_messages = gmail::messages::list(&access, &email)
            .map_err(|err| println!("{:#?}", err))
            .ok().unwrap();
        
        // Wait a few seconds and query the number of items in the inbox again
        let retry_time = time::Duration::from_secs(5);
        thread::sleep(retry_time);

        if n_retries > 240 { // 20 minutes
            panic!("Error: number of retries excceeded when parsing emails.");
        }
        
        // Check if messages list equals the expected number of messages
        if &res == &list_messages.result_size_estimate {
            println!("Expected messages are present");
            break;
        }

        println!("Waiting for emails to appear in inbox of {}", &email);
    }

    let list_messages = gmail::messages::list(&access, &email)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    // Loop throught emails and download csv files containing dataset
    for msg in &list_messages.messages {

        let message = gmail::messages::read_message(&access, &email, &msg.id)
            .map_err(|err| println!("Read msg: {}", err))
            .ok().unwrap();

        // message.snippet contains the body (atleast in short emails)
        let url = gmail::messages::extract_url(&message.snippet);
        let device_name = gmail::messages::extract_device_name(&message.snippet);

        data::download::csv(&url, &device_name)
            .map_err(|err| println!("Error downloading file: {}", err))
            .ok().unwrap();
    }

    // Takes the captured datasets and transforms them into a summary .csv
    // file that can be uploaded to datawrapper (this method runs transform.py)
    data::transform::to_csv();

    // Upload chart data and publish chart to datawrapper
    let table_id = env::var("TABLE_ID").expect("Table ID not found");
    let dw_key = env::var("DW_KEY").expect("Datawapper key not found");
    let dataset_path = String::from("data/transformed/transformed.csv");

    datawrapper::export::upload_dataset(&dataset_path, &table_id, &dw_key)
        .map_err(|err| println!("{}", err))
        .ok();

    datawrapper::export::publish_chart(&table_id, &dw_key)
        .map_err(|err| println!("{}", err))
        .ok();

    // Get the data from the AWS and publish it to datawrapper
    let aws_token = env::var("AWS_ORG_KEY").expect("AWS org key not found");
    let dw_key = env::var("DW_KEY").expect("Datawapper key not found");

    let aws = ubidots::device::aws::weekly_precipitation(&aws_token)
        .map_err(|err| println!("{}", err))
        .ok().expect("Precipitation parse error.");

    datawrapper::chart::json_to_csv(&aws);

    let aws_path = String::from("data/precipitation/precipitation.csv");
    let chart_id = env::var("CHART_ID").expect("Chart ID not found");
    datawrapper::export::upload_dataset(&aws_path, &chart_id, &dw_key)
        .map_err(|err| println!("{}", err))
        .ok();

    datawrapper::export::publish_chart(&chart_id, &dw_key)
        .map_err(|err| println!("{}", err))
        .ok();
}
