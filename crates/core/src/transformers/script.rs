use crate::http::request::Request;

pub async fn execute_sript(script: String, req: Request) -> anyhow::Result<Request> {
    let _ = js::execute_sript(script).await?;
    Ok(req)
}
