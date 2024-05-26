use std::time::Duration;

use anyhow::Ok;
use rustyscript::{Module, Runtime, RuntimeOptions};

async fn runjs(code: &str) -> anyhow::Result<()> {
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(5000),
        ..Default::default()
    })?;

    let module = Module::new("script.js", code);
    let _ = runtime.load_module(&module).await?;
    Ok(())
}

pub struct Request {
    pub method: String,
    pub url: String,
    pub path_params: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub query_params: Vec<(String, String)>,
    pub body: String,
    pub auth: Option<(String, String)>,
}

pub struct RequestScriptCtx {
    pub req: Request,
    pub script: String,
}

pub async fn execute_sript(script: String) -> anyhow::Result<()> {
    let (sx, mut rx) = tokio::sync::mpsc::channel(1);

    let thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let _ = runjs(&script).await;
            sx.send(()).await.unwrap();
        });
    });

    rx.recv().await;
    thread.join().unwrap();

    Ok(())
}
