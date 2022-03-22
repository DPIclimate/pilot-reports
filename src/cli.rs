//! Command line interface for pilot reports
use clap::{Arg, Command};
use log::info;

pub struct Config {
    /// Use data from cache (true) or from Ubidots (false)
    pub use_cache: bool,
}

impl Config {
    /// Create new cli `Config`
    pub fn new() -> Self {
        let args = Command::new("Pilot Reports")
            .version("0.1.0")
            .author("Harvey Bates <harvey.bates@dpi.nsw.gov.au>")
            .about("Automatic report generation for Climate Smart Pilots")
            .arg(
                Arg::new("refresh")
                    .short('r')
                    .long("refresh")
                    .takes_value(false)
                    .help("Request variables and devices from Ubidots. Rather than cache."),
            )
            .get_matches();

        if args.is_present("refresh") {
            info!("Refreshing data and saving it to cache");
            return Config { use_cache: false };
        }
        info!("Using cache data from cache directory");
        Config { use_cache: true }
    }
}
