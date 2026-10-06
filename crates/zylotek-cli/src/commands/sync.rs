use clap::Subcommand;

use zylotek_sync::Syncronizer;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Subcommand, Debug, PartialEq, Eq, Clone)]
pub enum SyncCommand {
    Run {
        #[arg(short, long)]
        config: Option<String>,
    },
}

pub fn execute(cmd: &SyncCommand) -> Result<(), Error> {
    match cmd {
        SyncCommand::Run { config } => cmd_run(config.as_deref()),
    }
}

fn cmd_run(_config: Option<&str>) -> Result<(), Error> {
    let syncronizer = Syncronizer::new();
    let syncronizer_runner = syncronizer.build();
    syncronizer_runner.run()
}
