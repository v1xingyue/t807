use {
    clap::{CommandFactory, Parser, Subcommand},
    log::{error, info},
};

/// 807 tools to manage binary packages on your system
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

pub fn cli_main() {
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
            error!("no detail sub command found");
            println!("{}", Cli::command().render_help());
        }
    }
}
