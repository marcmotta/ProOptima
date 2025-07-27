// src/main.rs
/*
 * Main executable for ProOptima
 */

use clap::Parser;
use prooptima::{Result, run};

#[derive(Parser)]
#[command(version, about = "ProOptima - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
