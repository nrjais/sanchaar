use std::path::PathBuf;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::persistence::request::EncodedRequest;

pub async fn save_req_to_file(path: PathBuf, req: EncodedRequest) -> Result<(), anyhow::Error> {
    let file = tokio::fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let encoded = toml::to_string_pretty(&req)?;

    writer.write_all(encoded.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_from_file(path: &PathBuf) -> Result<EncodedRequest, Box<dyn std::error::Error>> {
    let file = tokio::fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;
    let decoded: EncodedRequest = toml::from_str(&buffer)?;

    Ok(decoded)
}
