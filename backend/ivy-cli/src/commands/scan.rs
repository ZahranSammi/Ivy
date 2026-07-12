use clap::Args;

#[derive(Args)]
pub struct ScanArgs {
    #[arg(short, long)]
    pub target: String,
    
    #[arg(short, long, default_value = "normal")]
    pub intensity: String,
}

pub async fn run(_args: ScanArgs) -> anyhow::Result<()> {
    println!("Scan started...");
    Ok(())
}
