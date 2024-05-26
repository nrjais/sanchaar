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

pub enum RequestBody {
    Json(serde_json::Value),
    Text(String),
    Binary(Vec<u8>),
}

pub struct Request {
    pub method: String,
    pub url: String,
    pub path_params: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub query_params: Vec<(String, String)>,
    pub auth: Option<(String, String)>,
    pub body: RequestBody,
    pub body_raw: Option<String>,
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
            .expect("Failed to build runtime");

        rt.block_on(async {
            let _ = runjs(&script).await;
            sx.send(()).await.expect("Failed to send on channel");
        });
    });

    rx.recv().await;
    thread.join().expect("Failed to join thread");

    Ok(())
}
