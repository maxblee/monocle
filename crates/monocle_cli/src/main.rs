use clap::{Parser, Subcommand};
use color_eyre::Result;
use runserver::RunServerArgs;

pub(crate) mod router;
mod runserver;

#[derive(Debug, Parser)]
#[command(name = "Monocle")]
#[command(version = "0.1")]
#[command(about = "Helps you Manage the application")]
struct SearchCmd {
    #[command(subcommand)]
    command: Commands,
}

impl SearchCmd {
    async fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Runserver(conf) => conf.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the application server
    Runserver(RunServerArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cmd = SearchCmd::parse();
    cmd.run().await
}
