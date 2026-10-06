use clap::Subcommand;

use zylotek_infrastructure::persistence::cli;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Subcommand, Debug, PartialEq, Eq, Clone)]
pub enum DbCommand {
    Setup,
    Migrate,
}

pub fn execute(cmd: &DbCommand) -> Result<(), Error> {
    match cmd {
        DbCommand::Setup => cmd_cli(vec!["migration", "generate"]),
        DbCommand::Migrate => cmd_cli(vec!["migration", "apply"]),
    }
}

fn cmd_cli(args: Vec<&str>) -> Result<(), Error> {
    Ok(cli(args)?)
}
