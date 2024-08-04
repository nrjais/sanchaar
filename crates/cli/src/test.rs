use core::{
    assertions::{self, runner::MatcherResult},
    client::{create_client, send_request},
    http::environment::EnvironmentChain,
    persistence::request::read_request,
    transformers::request::transform_request,
};
use std::path::PathBuf;

use anyhow::Context;
use hcl::Value;

use crate::color::{color, Color};

pub async fn test(root: PathBuf, path: PathBuf) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let root = current_dir.join(root);

    let path = root.join(path);

    let client = create_client(false, Default::default());

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
    let file_name = path
        .file_name()
        .context("Invalid path")?
        .to_str()
        .context("Invalid file name")?;

    let req = read_request(path).await?;

    let assertions = req.assertions.clone();

    let req = transform_request(client.clone(), req, EnvironmentChain::new()).await?;
    let response = send_request(client, req).await?;

    let result = assertions::run(&response, &assertions);

    println!("{} - {} assertions", file_name, result.len());

    let indent = Indent::new();
    for assertion in result {
        let indent = indent.inc();
        println!("{:id$}Assert {}", "", assertion.name, id = indent.v);

        for cond in assertion.results {
            let indent = indent.inc();
            match cond.result {
                MatcherResult::Passed => {
                    let msg = format!("{:id$}{}", "", cond.name, id = indent.v);
                    println!("{}", color(&msg, Color::LIGHTGREEN));
                }
                MatcherResult::Failed(des) => {
                    let msg = format!("{:id$}{}", "", cond.name, id = indent.v);
                    println!("{}", color(&msg, Color::RED));
                    {
                        let indent = indent.inc();
                        let msg = format!("{:id$}Summary: {}", "", des.summary, id = indent.v);
                        println!("{}", color(&msg, Color::YELLOW));
                        println!(
                            "{:id$}Actual: {}",
                            "",
                            color(&des.actual.unwrap_or(Value::Null).to_string(), Color::RED),
                            id = indent.v
                        );
                        println!(
                            "{:id$}Expected: {}",
                            "",
                            color(&des.expected.to_string(), Color::LIGHTGREEN),
                            id = indent.v
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

struct Indent {
    v: usize,
}

impl Indent {
    fn new() -> Self {
        Self { v: 0 }
    }

    fn inc(&self) -> Self {
        Self { v: self.v + 2 }
    }
}
