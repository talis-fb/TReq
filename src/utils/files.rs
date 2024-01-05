use std::path::PathBuf;
use anyhow::Result;

pub async fn read_to_string(path: PathBuf) -> Result<String> {
    tokio::fs::read_to_string(&path).await?
}
