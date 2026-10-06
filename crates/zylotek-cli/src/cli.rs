use clap::{Parser, Subcommand};

use crate::commands::{db::DbCommand, sync::SyncCommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Commands {
    #[command(subcommand)]
    Db(DbCommand),

    #[command(subcommand)]
    Sync(SyncCommand),
}
