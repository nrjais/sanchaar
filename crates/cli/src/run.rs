use core::{
    client::{create_client, send_request, Response},
    persistence::request::read_request,
    transformers::request::transform_request,
    utils::fmt_duration,
};
use std::{env, path::PathBuf};

use humansize::{format_size, BINARY};

pub async fn run(root: PathBuf, req: PathBuf, verbose: bool) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let root = current_dir.join(root);

    let path = root.join(req);
    let req = read_request(path).await?;

    let client = create_client();
    let req = transform_request(client.clone(), req, None).await?;
    let response = send_request(client, req).await?;

    let Response {
        status,
        headers,
        body,
        duration,
        size_bytes,
    } = response;
    if verbose {
        println!("{}", status);
        println!("Duration: {}", fmt_duration(duration));
        println!("Size: {}", format_size(size_bytes, BINARY));

        println!();
        if !headers.is_empty() {
            println!("Headers:");
            for (k, v) in headers.iter() {
                let value = v.to_str().unwrap_or("<Invalid UTF-8>");
                println!("  {}: {}", k.as_str(), value);
            }
        }

        println!();
        match body.content_type {
            core::client::ContentType::Json => {
                println!("Body JSON:");
            }
            core::client::ContentType::Text => {
                println!("Body Text:");
            }
            core::client::ContentType::Buffer => {
                println!("Body Buffer (Hex):");
            }
        }
    }

    match body.content_type {
        core::client::ContentType::Json => {
            let json: serde_json::Value = serde_json::from_slice(&body.data)?;
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        core::client::ContentType::Text => {
            let text = String::from_utf8(body.data)?;
            println!("{}", text);
        }
        core::client::ContentType::Buffer => {
            let hex = hex::encode(body.data);
            println!("{}", hex);
        }
    }

    Ok(())
}
