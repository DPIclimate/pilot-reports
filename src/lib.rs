// use cargo run -- --nocapture to see println! statements


#[cfg(test)]
mod tests {
    use super::*;
    extern crate dotenv;
    use std::env;

    mod ubidots;
    mod gmail;
    mod data;
    mod datawrapper;
    mod utils;


    #[test]
    fn test_download_csv() {
        // Test the process of downloading a csv file from a specified url
        let url = "https://s3.amazonaws.com/prd-293huhzkha/00d11feca77ff462_device_7_variables_5lbO.csv".to_string();
        let fname = "clyde-salinity90d".to_string();
        data::download::csv(&url, &fname)
            .map_err(|err| println!("Error downloading file: {}", err))
            .ok().unwrap();
    }

    #[test]
    fn test_extract_device_name() {
        // Test the extraction of device name from email contents
        // Has the potential to change in the future if ubidots updates their mail strucutre
        // In that case, this funciton will need to be changed accordingly
        let msg = "Hi there, Your sensor data export &quot;clyde-salinity90b&quot; is ready for download https://s3.amazonaws.com/prd-293huhzkha/00e3ced15d56e7a9_device_7_variables_Xqjl.csv All the best,".to_string();

        let name = gmail::messages::extract_device_name(&msg);

        assert_eq!("clyde-salinity90b", name);
    }

    #[test]
    fn test_python_transform() {
        // Test taking the devices .csv files and transform them into the datawrapper template
        data::transform::to_csv();
    }

    #[test]
    fn test_datawrapper_upload() {
        // Test the process of uploading .csv dataset to datawrapper and 
        // publishing the chart
        dotenv::dotenv().expect("Failed to read .env file");
        let table_id = env::var("TABLE_ID").expect("Table ID not found");
        let dw_key = env::var("DW_KEY").expect("Datawapper key not found");
        let dataset_path = String::from("data/transformed/transformed.csv");
        datawrapper::export::upload_dataset(&dataset_path, &table_id, &dw_key)
            .map_err(|err| println!("{}", err))
            .ok();

        datawrapper::export::publish_chart(&table_id, &dw_key)
            .map_err(|err| println!("{}", err))
            .ok();
    }

    #[test]
    fn test_precipitation_chart_create() {
        dotenv::dotenv().expect("Failed to read .env file");
        
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
}


