use clap::{Arg, Command};
use log::info;

pub struct Config {
    pub use_cache: bool,
}

impl Config {
    pub fn new() -> Self {
        let args = Command::new("Pilot Reports")
            .version("0.1.0")
            .author("Harvey Bates <harvey.bates@dpi.nsw.gov.au>")
            .about("Automatic report generation for Climate Smart Pilots")
            .arg(Arg::new("use-cache")
                .short('c')
                .long("usecache")
                .takes_value(false)
                .help("Use stored cache variables and devices rather than requesting them from Ubidots.",
            ))
            .get_matches();

        if args.is_present("use-cache") {
            info!("Using cache data from cache directory");
            return Config { use_cache: true };
        }
        info!("Refreshing data and saving it to cache");
        Config { use_cache: true }
    }
}
