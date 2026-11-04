//! Runtime daemon for rudderless.

use anyhow::Result;
use clap::Parser;
use librdl::VERSION;
use tracing::info;

/// Rudderless daemon.
#[derive(Debug, Parser)]
#[command(name = "rdld", version = VERSION, about = "Rudderless daemon")]
struct Cli {
    /// Print version and exit.
    #[arg(long)]
    version: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    if cli.version {
        println!("rdld {}", VERSION);
        return Ok(());
    }

    info!("rdld {} starting", VERSION);
    println!("rdld {} ready", VERSION);
    Ok(())
}
