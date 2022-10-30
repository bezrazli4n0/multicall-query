mod abi;
pub mod state;
mod utils;

use anyhow::Result;
use ethers::{abi as ethers_abi, prelude::*};

pub async fn run(config: &state::Config) -> Result<()> {
    let client = config.get_client();
    let multicall = abi::IMultiCallABI::new(config.get_multicall_address(), client);

    let target_abi = config.get_target_abi();
    let target_addresses = config.get_target_addresses();
    let target_func = target_abi.function(&config.get_target_func_name())?;

    let payload_func = target_func.encode_input(&[])?;
    let payload: Vec<(Address, Bytes)> = target_addresses
        .iter()
        .map(|address| (*address, payload_func.clone().into()))
        .collect();

    let (block_number, results): (U256, Vec<Bytes>) = multicall.aggregate(payload).call().await?;

    let results: Vec<(String, Vec<ethers_abi::Token>)> = results
        .iter()
        .zip(target_addresses)
        .map(|(output, address)| {
            (
                format!("{:#02x}", address),
                target_func
                    .decode_output(output)
                    .expect("Unable to decode(abi) func output"),
            )
        })
        .collect();

    utils::export_results(
        block_number,
        results,
        config.get_export_format(),
        config.get_export_path(),
    )?;

    Ok(())
}
