mod args;

use args::Args;
use clap::Parser;
use multicall_query::Config;
use std::process;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::build(args.url).unwrap_or_else(|error| {
        eprintln!("Unable to create `Config`: {error}");
        process::exit(1);
    });

    if let Err(error) = multicall_query::run(&config).await {
        eprintln!("Unable to `run` application: {error}");
        process::exit(1);
    }
}
