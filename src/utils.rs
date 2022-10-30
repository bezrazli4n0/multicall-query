use crate::state::ExportFormat;
use anyhow::Error;
use ethers::{abi, prelude::*};
use std::{fs, io};

pub fn export_results(
    block_number: U256,
    results: Vec<(String, Vec<abi::Token>)>,
    export_format: ExportFormat,
    export_path: impl AsRef<str>,
) -> Result<(), Error> {
    let export_file = fs::File::create(export_path.as_ref())?;
    let export_file_wr = io::BufWriter::new(export_file);

    match export_format {
        ExportFormat::Json => serde_json::to_writer(
            export_file_wr,
            &serde_json::json!({
                "block_number": block_number.as_u64(),
                "results": results
            }),
        )?,
    }

    Ok(())
}
