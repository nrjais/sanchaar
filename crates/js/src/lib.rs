use std::time::Duration;

use anyhow::Ok;
use rustyscript::{Module, Runtime, RuntimeOptions};

pub async fn main() -> anyhow::Result<()> {
    let mut runtime = Runtime::new(RuntimeOptions {
        // default_entrypoint: Some("load".to_string()),
        timeout: Duration::from_millis(50),
        ..Default::default()
    })?;

    let module = Module::new("test.js", "console.log('Hello from Rust!'); var test=5");

    let handle = runtime.load_module(&module).await?;

    let res: usize = runtime.get_value(&handle, "test").await?;

    println!("Result: {:?}", res);

    Ok(())
}
