use anyhow::Result;
use ethers::{prelude::*, providers::Provider};

#[derive(Debug, Clone)]
pub struct Config {
    client: Provider<Http>,
}

impl Config {
    pub fn build(rpc_url: impl AsRef<str>) -> Result<Self> {
        Ok(Config {
            client: Provider::<Http>::try_from(rpc_url.as_ref())?,
        })
    }
}
