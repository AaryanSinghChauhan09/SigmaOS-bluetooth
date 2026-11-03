use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use bluetooth::bluetooth;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = bluetooth::Config { debug: cli.debug, ..Default::default() };
    let mut component = bluetooth::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-bluetooth running (Ctrl+C to stop)");
    tokio::runtime::Runtime::new()?.block_on(async {
        tokio::signal::ctrl_c().await.unwrap();
    });
    
    component.stop()?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
