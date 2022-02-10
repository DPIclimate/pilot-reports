extern crate dotenv;
use chrono::{NaiveDate, NaiveDateTime, Local, offset::TimeZone};
use std::env;

mod ubidots;
mod gmail;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let email = env::var("GMAIL_ADDR").expect("Email address not found");

    let access = gmail::authenticate::run();

    println!("Auth-Token: {}", &access.as_str());

    let list_messages = gmail::messages::list(&access, &email)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    if list_messages.result_size_estimate != 0 as i64 {
        for msg in &list_messages.messages {

            println!("Msg id: {}", &msg.id);

            let message = gmail::messages::read_message(&access, &email, &msg.id)
                .map_err(|err| println!("Read msg: {}", err))
                .ok().unwrap();

            let url = gmail::messages::extract_url(&message);

            println!("Url: {}", url);
        }
    }

//    gmail::messages::clear_inbox(&access, &email, &list_messages)
//        .map_err(|err| println!("{:#?}", err))
//        .ok();
//
    
    // Dotenv setup
//    let org_token = env::var("ORG_KEY").expect("Organisation key not found");
//
//    // Specify start and end times
//    let start_date = NaiveDate::from_ymd(2021, 2, 1)
//        .and_hms(0, 0, 0)
//        .timestamp() * 1000;
//    let end_date = Local::now().timestamp() * 1000;
//    let date_range = (start_date, end_date);
//
//    // Export all data from an org to an email (defined in .env)
//    let res = ubidots::export::data_to_email(&org_token, &email, &date_range);
}
