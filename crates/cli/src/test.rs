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

    let req = read_request(path).await?;

    let assertions = req.assertions.clone();

    let client = create_client();
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
