use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    debug!("debug mesage");
    error!("Hello, error world!");
    info!("rust info ..");
    info!("rust info log_enabled : {}", log_enabled!(Level::Info));
}
