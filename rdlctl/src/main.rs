//! Control CLI for rudderless.

use anyhow::Result;
use clap::Parser;
use librdl::VERSION;

/// Rudderless control CLI.
#[derive(Debug, Parser)]
#[command(name = "rdlctl", version = VERSION, about = "Rudderless control CLI")]
struct Cli {
    /// Print version and exit.
    #[arg(long)]
    version: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.version {
        println!("rdlctl {}", VERSION);
        return Ok(());
    }

    println!("rdlctl {}", VERSION);
    Ok(())
}
