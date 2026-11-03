use anyhow::Result;
use clap::Parser;
use podman::podman;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = podman::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = podman::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-podman deployed");
    Ok(())
}
