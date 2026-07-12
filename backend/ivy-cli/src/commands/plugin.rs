use clap::{Args, Subcommand};

#[derive(Args)]
pub struct PluginArgs {
    #[command(subcommand)]
    pub command: PluginCommand,
}

#[derive(Subcommand)]
pub enum PluginCommand {
    Install { name: String },
    Remove { name: String },
    List,
}

pub async fn run(_args: PluginArgs) -> anyhow::Result<()> {
    println!("Plugin command executed.");
    Ok(())
}
