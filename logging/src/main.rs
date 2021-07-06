// RUST_LOG=trace cargo run

use log::{trace, debug, info, warn, error};

fn main() {
    env_logger::init();
    trace!("logger initialized");
    debug!("about to send info message");
    info!("Hello, logger.");
    warn!("Maybe gonna send an error message");
    error!("Problem!");
}
