mod config;
mod constant;
mod server;
mod args;
mod entry;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let process_args = args::Args::parse();
    let config = config::read_config(&process_args.config)?;

    Ok(())
}
