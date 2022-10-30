use super::ExportFormat;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Node RPC URL.
    #[arg(short, long)]
    pub url: String,

    /// Multicall contract address.
    #[arg(short, long, default_value_t = String::from("0xeefba1e63905ef1d7acba5a8513c70307c1ce441"))]
    pub multicall_address: String,

    /// Path for file with target addresses.
    ///
    /// Each address should be separated by newline(`\n`).
    #[arg(long)]
    pub target_addrs_path: String,

    /// Path for json file with target ABI.
    #[arg(long)]
    pub target_abi_path: String,

    /// Target function name from ABI (without args / params).
    #[arg(long)]
    pub target_func_name: String,

    /// Export format.
    #[arg(short, long, default_value_t = ExportFormat::Json)]
    pub export: ExportFormat,

    // Export output path.
    #[arg(long, default_value_t = String::from("output.json"))]
    pub export_path: String,
}
