// use cargo run -- --nocapture to see println! statements

#[cfg(test)]
mod tests {
    extern crate dotenv;
    use std::env;

    mod data;
    mod utils;

    #[test]
    fn load_config() {
        let config = utils::config::get_config()
            .map_err(|err| println!("Error loading config: {}", err))
            .ok().unwrap();

        for device in &config.devices {
            println!("Device name: {}", device.name);
        }

        for name in &config.variables {
            println!("Variable name: {}", name);
        }
    }

    #[test]
    fn unix_timestamp_to_local_day(){
        let ts = 1645491182126;
        let local_day = utils::time::unix_to_local(&ts)
            .date()
            .format("%A");
        assert_eq!("Tuesday".to_string(), local_day.to_string());
    }

    #[test]
    fn create_csv_files() {
        data::files::create_output_csv_files();
    }
}

