// src/main.rs
/*
 * Main executable for SolidityCryptoSDKToolkitUltra
 */

use clap::Parser;
use soliditycryptosdktoolkitultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "SolidityCryptoSDKToolkitUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
