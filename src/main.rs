mod domain;
mod infrastructure;
mod tests;

use clap::Parser;
use crate::infrastructure::cli_handler::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}