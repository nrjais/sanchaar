use std::time::Duration;

use anyhow::Ok;
use rustyscript::{Module, Runtime, RuntimeOptions};

pub async fn runjs(code: &str) -> anyhow::Result<()> {
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(5000),
        ..Default::default()
    })?;

    let module = Module::new("script.js", code);
    let _ = runtime.load_module(&module).await?;
    Ok(())
}
