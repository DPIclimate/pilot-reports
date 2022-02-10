//! Export data to an email address
use crate::ubidots::device;

pub fn data_to_email(token: &String, email: &String, date_range: &(i64, i64)){
    // Export all devices to email within a set date range
    // Ubidots requires a post request to get an email containing a .csv file of device data
    // This function triggers the email to be sent for all devices within a device
    // The email has to be handled hence the use of a gmail module

    let devices = device::get_all(&token)
        .map_err(|err| println!("{:#?}", err))
        .ok().unwrap();

    for dev in &devices.results {
        let api_label = device::Identifier::ApiLabel(String::from(&dev.label));
        println!("{:#?}", &dev.label);
        let res = device::variables::values::get(&api_label, &token, &email, &date_range)
            .map_err(|err| println!("{:#?}", err))
            .ok().unwrap();
    }
}
