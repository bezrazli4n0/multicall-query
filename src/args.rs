use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Node RPC URL.
    #[arg(short, long)]
    pub url: String,
}
