use {
    clap::{Parser, Subcommand},
    env_logger::Env,
    log::{debug, error, info, log_enabled, Level},
};

/// 807 tools to manage binary packages on your system
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install binanry package
    Install {
        name: String,
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Update binanry package
    Update { name: String },
    /// Update binanry package
    List {},
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cmd: Cli = Cli::parse();
    match cmd {
        Cli {
            command: Some(Commands::Install { list, ref name }),
        } => {
            info!("do something with name : {}", name);
            if list {
                println!("list");
            } else {
                println!("install");
            }
        }
        Cli {
            command: Some(Commands::Update { ref name }),
        } => {
            info!("do something update with name : {}", name);
            println!("upgrade");
        }
        Cli {
            command: Some(Commands::List {}),
        } => {
            info!("list package installed with 807 tools");
            println!("list");
        }
        _ => {
            error!("no command found");
        }
    }
    // println!("Hello, {:?}!", cmd);

    // debug!("debug mesage");
    // error!("Hello, error world!");
    // info!("rust info ..");
    // info!("rust info log_enabled : {}", log_enabled!(Level::Info));
}
