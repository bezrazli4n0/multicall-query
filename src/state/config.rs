use super::{Args, ExportFormat};
use anyhow::Result;
use ethers::{abi::Abi, prelude::*, providers::Provider};
use std::{
    fs,
    io::{self, prelude::*},
    sync,
};

#[derive(Debug, Clone)]
pub struct Config {
    client: sync::Arc<Provider<Http>>,
    multicall_address: Address,
    export_format: ExportFormat,
    export_path: String,
    target_addresses: Vec<Address>,
    target_abi: Abi,
    target_func_name: String,
}

impl Config {
    pub fn build(args: &Args) -> Result<Self> {
        let target_addrs_file = fs::File::open(&args.target_addrs_path)?;
        let target_abi_file = fs::File::open(&args.target_abi_path)?;

        let target_addrs_rdr = io::BufReader::new(target_addrs_file);
        let target_abi_rdr = io::BufReader::new(target_abi_file);

        Ok(Config {
            client: sync::Arc::new(Provider::<Http>::try_from(&args.url)?),
            multicall_address: args.multicall_address.parse::<Address>()?,
            export_format: args.export,
            export_path: args.export_path.clone(),
            target_addresses: target_addrs_rdr
                .lines()
                .filter_map(|maybe_addr_line| {
                    if let Ok(maybe_addr_str) = maybe_addr_line {
                        if let Ok(addr) = maybe_addr_str.parse::<Address>() {
                            Some(addr)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            target_abi: Abi::load(target_abi_rdr)?,
            target_func_name: args.target_func_name.to_string(),
        })
    }

    pub fn get_multicall_address(&self) -> Address {
        self.multicall_address
    }

    pub fn get_export_format(&self) -> ExportFormat {
        self.export_format
    }

    pub fn get_export_path(&self) -> String {
        self.export_path.clone()
    }

    pub fn get_target_abi(&self) -> Abi {
        self.target_abi.clone()
    }

    pub fn get_target_func_name(&self) -> String {
        self.target_func_name.clone()
    }

    pub fn get_target_addresses(&self) -> Vec<Address> {
        self.target_addresses.clone()
    }

    pub fn get_client(&self) -> sync::Arc<Provider<Http>> {
        sync::Arc::clone(&self.client)
    }
}
