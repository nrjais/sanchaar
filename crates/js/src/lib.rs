pub mod request;

use std::time::Duration;

use anyhow::Ok;
pub use request::Request;
pub use request::RequestBody;
use rustyscript::{Module, Runtime, RuntimeOptions};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

async fn runjs(code: &str, state: Request) -> anyhow::Result<()> {
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(5000),
        ..Default::default()
    })?;

    runtime.put(state).expect("Failed to put state");

    let module = Module::new("sanchaar_pre_request.ts", code);
    let _ = runtime.load_module(&module).await?;
    Ok(())
}

pub struct RequestScriptCtx {
    pub request: Request,
    pub script: String,
}

pub async fn execute_sript(ctx: RequestScriptCtx) -> anyhow::Result<()> {
    let (sx, mut rx) = tokio::sync::mpsc::channel(1);

    let RequestScriptCtx { script, request } = ctx;

    let thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build runtime");

        rt.block_on(async {
            let _ = runjs(&script, request).await;
            sx.send(()).await.expect("Failed to send on channel");
        });
    });

    rx.recv().await;
    thread.join().expect("Failed to join thread");

    Ok(())
}
