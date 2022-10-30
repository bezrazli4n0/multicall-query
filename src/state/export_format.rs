use anyhow::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
}

impl ToString for ExportFormat {
    fn to_string(&self) -> String {
        match self {
            ExportFormat::Json => "json".to_string(),
        }
    }
}

impl FromStr for ExportFormat {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_ref() {
            "json" => Ok(ExportFormat::Json),
            _ => anyhow::bail!("Invalid `ExportFormat` type"),
        }
    }
}
