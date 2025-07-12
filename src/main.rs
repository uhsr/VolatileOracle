// src/main.rs
/*
 * Main executable for VolatileOracle
 */

use clap::Parser;
use volatileoracle::{Result, run};

#[derive(Parser)]
#[command(version, about = "VolatileOracle - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
