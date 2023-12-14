use {
    env_logger::Env,
    t817::{cli, info, sui},
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    sui::init_account();
    cli::cli_main();
    info::description();
}
