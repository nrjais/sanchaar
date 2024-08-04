use colored_json::prelude::ToColoredJson;
use core::{
    client::{create_client, send_request, ContentType, Response},
    http::environment::EnvironmentChain,
    persistence::request::read_request,
    transformers::request::transform_request,
    utils::fmt_duration,
};
use std::{env, path::PathBuf, sync::Arc};

use humansize::{format_size, BINARY};

use crate::color::{color, Color};

pub async fn run(root: PathBuf, req: PathBuf, verbose: bool) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let root = current_dir.join(root);

    let path = root.join(req);
    let req = read_request(&path).await?;

    let client = create_client(false, Default::default());
    let req = transform_request(client.clone(), req, EnvironmentChain::new()).await?;
    let response = send_request(client, req).await?;

    let Response {
        status,
        headers,
        body,
        duration,
        size_bytes,
    } = response;
    if verbose {
        println!("{}", color(&status.to_string(), Color::CYAN));
        println!(
            "{} {}",
            color("Size:", Color::DARKGRAY),
            color(&format_size(size_bytes, BINARY), Color::VIOLET)
        );
        println!(
            "{} {}",
            color("Time:", Color::DARKGRAY),
            color(&fmt_duration(duration), Color::VIOLET)
        );

        println!();
        if !headers.is_empty() {
            println!("{}", color("Headers:", Color::DARKGRAY));
            for (k, v) in headers.iter() {
                let value = v.to_str().unwrap_or("<Invalid UTF-8>");
                println!(
                    "  {}: {}",
                    color(k.as_str(), Color::BLUE),
                    color(value, Color::DARKGREEN)
                );
            }
        }
    }

    println!();
    let data = Arc::unwrap_or_clone(body.data.clone());
    match body.content_type {
        ContentType::Json => {
            let json = String::from_utf8(data)?;
            println!("{}", json.to_colored_json_auto()?);
        }
        ContentType::Text => {
            let text = String::from_utf8(data)?;
            println!("{}", text);
        }
        ContentType::Buffer => {
            let hex = hex::encode(data);
            println!("Hex:\n{}", hex);
        }
    }

    Ok(())
}
