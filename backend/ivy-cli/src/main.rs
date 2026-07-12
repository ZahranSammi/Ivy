mod commands;

use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "ivy", version, about = "Ivy OSINT CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new scan
    Scan(commands::scan::ScanArgs),
    /// Manage plugins
    Plugin(commands::plugin::PluginArgs),
    /// Manage configuration
    Config(commands::config::ConfigArgs),
    /// Export graph data
    Export(commands::export::ExportArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan(args) => commands::scan::run(args).await?,
        Commands::Plugin(args) => commands::plugin::run(args).await?,
        Commands::Config(args) => commands::config::run(args).await?,
        Commands::Export(args) => commands::export::run(args).await?,
    }

    Ok(())
}
