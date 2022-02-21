// use cargo run -- --nocapture to see println! statements

mod data;
mod gmail;

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    //#[test]
    fn test_download_csv() {
        // Test the process of downloading a csv file from a specified url
        let url = "https://s3.amazonaws.com/prd-293huhzkha/00d11feca77ff462_device_7_variables_5lbO.csv".to_string();
        let fname = "clyde-salinity90d".to_string();
        data::download::csv(&url, &fname)
            .map_err(|err| println!("Error downloading file: {}", err))
            .ok().unwrap();
    }

    //#[test]
    fn test_extract_device_name() {
        // Test the extraction of device name from email contents
        // Has the potential to change in the future if ubidots updates their mail strucutre
        // In that case, this funciton will need to be changed accordingly
        let msg = "Hi there, Your sensor data export &quot;clyde-salinity90b&quot; is ready for download https://s3.amazonaws.com/prd-293huhzkha/00e3ced15d56e7a9_device_7_variables_Xqjl.csv All the best,".to_string();

        let name = gmail::messages::extract_device_name(&msg);

        assert_eq!("clyde-salinity90b", name);
    }


    #[test]
    fn test_data_modify(){
        let df = data::transform::to_summary().unwrap();
    
        let sal_iter = &df["salinity"].f64().unwrap().into_iter()
            .for_each(|opt_v| println!("{:?}", opt_v));



    }
}
