use clap::Subcommand;

use zylotek_api::cli;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Subcommand, Debug, PartialEq, Eq, Clone)]
pub enum ApiCommand {
    Run,
}

pub fn execute(cmd: &ApiCommand) -> Result<(), Error> {
    match cmd {
        ApiCommand::Run => cmd_cli(vec![]),
    }
}

fn cmd_cli(args: Vec<&str>) -> Result<(), Error> {
    Ok(cli(args)?)
}
