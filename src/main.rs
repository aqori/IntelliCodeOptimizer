// src/main.rs
/*
 * Main executable for IntelliCodeOptimizer
 */

use clap::Parser;
use intellicodeoptimizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliCodeOptimizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
