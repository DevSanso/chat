mod config;
mod constant;
mod server;
mod args;

use clap::Parser;

fn main() {
    let process_args = args::Args::parse();
    
}
