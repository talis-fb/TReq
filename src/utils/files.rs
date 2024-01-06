pub mod file_utils {
    use std::path::PathBuf;

    use anyhow::Result;
    use tokio::io::AsyncWriteExt;

    pub async fn read_from_file(path: PathBuf) -> Result<String> {
        Ok(tokio::fs::read_to_string(&path).await?)
    }

    pub async fn write_to_file(path: PathBuf, content: &str) -> Result<()> {
        Ok(tokio::fs::write(&path, content.as_bytes()).await?)
    }

    pub async fn append_to_file(path: PathBuf, content: &str) -> Result<()> {
        tokio::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .await?
            .write_all(content.as_bytes())
            .await?;
        Ok(())
    }
}
