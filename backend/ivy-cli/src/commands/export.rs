use clap::Args;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(short, long)]
    pub project_id: String,
    
    #[arg(short, long, default_value = "json")]
    pub format: String,
}

pub async fn run(_args: ExportArgs) -> anyhow::Result<()> {
    println!("Export command executed.");
    Ok(())
}
