mod cli;
mod commands;

use clap::Parser;

use cli::{Cli, Commands};
use commands::{db, sync};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let cli = Cli::parse();

    let result = match cli.command {
        None => {
            let _ = Cli::parse_from(["zylotek", "--help"]);
            Ok(())
        }
        Some(Commands::Db(cmd)) => db::execute(&cmd),
        Some(Commands::Sync(cmd)) => sync::execute(&cmd),
    };

    if let Err(err) = result {
        eprintln!("ERROR: {:#?}", err);
        std::process::exit(1);
    }
}
