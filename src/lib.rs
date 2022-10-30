mod config;

use anyhow::Result;
pub use config::Config;

pub async fn run(_config: &Config) -> Result<()> {
    Ok(())
}
