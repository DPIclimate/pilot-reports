extern crate dotenv;
use chrono::Local;
use std::env;

mod ubidots;
mod gmail;
mod data;

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
    
    // Specify start and end times
    let end_date = Local::now().timestamp() * 1000;
    let start_date = &end_date - 604800000; // One week ago
    let date_range = (start_date, end_date);

    let devices = ubidots::device::get_all(&org_token)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    // Export all data from an org to an email (defined in .env)
    let res = ubidots::export::data_to_email(&org_token, &email, &devices, &date_range)
        .map_err(|err| println!("Email error: {}", err))
        .ok().unwrap();

//    loop {
//        let list_messages = gmail::messages::list(&access, &email)
//            .map_err(|err| println!("{:#?}", err))
//            .ok().unwrap();
//    }
//
//
//    println!("Auth-Token: {}", &access.as_str());
//
//
//    if list_messages.result_size_estimate != 0 as i64 {
//        for msg in &list_messages.messages {
//
//            println!("Msg id: {}", &msg.id);
//
//            let message = gmail::messages::read_message(&access, &email, &msg.id)
//                .map_err(|err| println!("Read msg: {}", err))
//                .ok().unwrap();
//
//            let url = gmail::messages::extract_url(&message);
//
//            println!("Url: {}", url);
//            
//            let fname = data::download::csv(&url).expect("Error creating file");
//        }
//    }

    // Dotenv setup
//
}
