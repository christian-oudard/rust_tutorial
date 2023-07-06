#[macro_use]
extern crate log;

use simplelog::*;
use std::fs::File;

fn main() {
    WriteLogger::init(LevelFilter::Info, Config::default(), File::create("log").unwrap()).unwrap();
    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}
