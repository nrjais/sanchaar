use core::{
    assertions::{self, runner::AssertionResult},
    client::{create_client, send_request},
    persistence::request::read_request,
    transformers::request::transform_request,
};
use std::path::PathBuf;

pub async fn test(root: PathBuf, path: PathBuf) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let root = current_dir.join(root);

    let path = root.join(path);

    let client = create_client();

    let file = tokio::fs::File::open(&path).await?;
    if file.metadata().await?.is_dir() {
        walk_dir(client, &path).await?;
    } else {
        test_file(client, &path).await?;
    }

    Ok(())
}

async fn walk_dir(client: reqwest::Client, path: &PathBuf) -> anyhow::Result<()> {
    let mut entries = tokio::fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file = tokio::fs::File::open(&entry_path).await?;
        if file.metadata().await?.is_dir() {
            Box::pin(walk_dir(client.clone(), &entry_path)).await?;
        } else {
            test_file(client.clone(), &entry_path).await?;
        }
    }

    Ok(())
}

async fn test_file(client: reqwest::Client, path: &PathBuf) -> anyhow::Result<()> {
    let req = read_request(path).await?;

    let assertions = req.assertions.clone();

    let req = transform_request(client.clone(), req, None).await?;
    let response = send_request(client, req).await?;

    let result = assertions::run(&response, &assertions);

    println!("Ran {} assertions", result.len());

    for assertion in result {
        println!();
        match &assertion.result {
            AssertionResult::Passed => {
                println!("{}", &assertion.name);
            }
            AssertionResult::Failed(msg) => {
                println!("{}", &assertion.name);
                println!("  {}", msg);
            }
        }
    }

    Ok(())
}
