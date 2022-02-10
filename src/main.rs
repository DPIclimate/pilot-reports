extern crate dotenv;
use chrono::{NaiveDate, NaiveDateTime, Local, offset::TimeZone};
use std::env;

mod ubidots;

fn main() {
    // Dotenv setup
    dotenv::dotenv().expect("Failed to read .env file");
    let org_token = env::var("ORG_KEY").expect("Organisation key not found");
    let email = env::var("EMAIL").expect("Email address not found");

    // Specify start and end times
    let start_date = NaiveDate::from_ymd(2021, 2, 1)
        .and_hms(0, 0, 0)
        .timestamp() * 1000;
    let end_date = Local::now().timestamp() * 1000;
    let date_range = (start_date, end_date);

    // Export all data from an org to an email (defined in .env)
    let res = ubidots::export::data_to_email(&org_token, &email, &date_range);
}
