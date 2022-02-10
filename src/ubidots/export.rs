//! Export data to an email address
use crate::ubidots::device;

pub fn data_to_email(token: &String, email: &String, date_range: &(i64, i64)){

    // Get devices of a specified org (via the org_token supplied)
    let devices = device::get_all(&token)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    for dev in &devices.results {
        let api_label = device::Identifier::ApiLabel(String::from(&dev.label));
        println!("{:#?}", &dev.label);
        let res = device::variables::values::get(&api_label, &token, &email, &date_range)
            .map_err(|err| println!("{:#?}", err))
            .ok().unwrap();
        break;
    }
}
