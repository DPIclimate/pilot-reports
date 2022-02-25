pub mod time {
    extern crate chrono;
    use chrono::prelude::*;

    pub fn unix_to_local(unix_time: &i64) -> DateTime<Local> {
        // Takes a unix time in ms (conveting it to seconds before parsing)
        // Returns the local time 

        let datetime_ts = Utc.timestamp(unix_time / 1000, 0);
        DateTime::<Local>::from(datetime_ts)

    }
}


