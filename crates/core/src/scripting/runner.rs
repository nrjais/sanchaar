use super::{ScriptEngine, ScriptRequest, ScriptResponse, ScriptVariables};
use crate::client::Response;
use crate::http::request::Request;
use anyhow::{Context, Result};
use std::time::Duration;

/// Pre-request script executor
pub struct PreRequestRunner {
    engine: ScriptEngine,
    variables: ScriptVariables,
}

impl PreRequestRunner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: ScriptEngine::new()?,
            variables: ScriptVariables::new(),
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.engine = self.engine.with_timeout(timeout);
        self
    }

    pub fn with_variables(mut self, variables: ScriptVariables) -> Self {
        self.variables = variables;
        self
    }

    /// Execute pre-request script and return modified request
    pub fn execute(&mut self, script: &str, request: &Request) -> Result<Request> {
        // Convert Request to ScriptRequest
        let mut script_req = ScriptRequest::new(
            request.method,
            request.url.clone(),
            request.headers.clone(),
            request.query_params.clone(),
        );

        // Execute script
        self.engine
            .execute_pre_request(script, &mut script_req, &mut self.variables)
            .context("Pre-request script execution failed")?;

        // Convert back to Request
        Ok(Request {
            description: request.description.clone(),
            method: script_req.method,
            url: script_req.url,
            headers: script_req.headers,
            query_params: script_req.query_params,
            path_params: request.path_params.clone(),
            body: request.body.clone(),
            auth: request.auth.clone(),
            assertions: request.assertions.clone(),
            pre_request: request.pre_request.clone(),
            post_request: request.post_request.clone(),
        })
    }

    /// Get variables after script execution
    pub fn variables(&self) -> &ScriptVariables {
        &self.variables
    }

    /// Get mutable variables
    pub fn variables_mut(&mut self) -> &mut ScriptVariables {
        &mut self.variables
    }
}

/// Post-request script executor
pub struct PostRequestRunner {
    engine: ScriptEngine,
    variables: ScriptVariables,
}

impl PostRequestRunner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: ScriptEngine::new()?,
            variables: ScriptVariables::new(),
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.engine = self.engine.with_timeout(timeout);
        self
    }

    pub fn with_variables(mut self, variables: ScriptVariables) -> Self {
        self.variables = variables;
        self
    }

    /// Execute post-request script
    pub fn execute(&mut self, script: &str, response: &Response) -> Result<()> {
        // Convert Response to ScriptResponse
        let script_resp = ScriptResponse {
            status: response.status.as_u16(),
            headers: response
                .headers
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
                .collect(),
            body: String::from_utf8_lossy(&response.body.data).to_string(),
            duration_ms: response.duration.as_millis() as u64,
        };

        // Execute script
        self.engine
            .execute_post_request(script, &script_resp, &mut self.variables)
            .context("Post-request script execution failed")?;

        Ok(())
    }

    /// Get variables after script execution
    pub fn variables(&self) -> &ScriptVariables {
        &self.variables
    }

    /// Get mutable variables
    pub fn variables_mut(&mut self) -> &mut ScriptVariables {
        &mut self.variables
    }
}

/// Run pre-request script on a request
pub fn run_pre_request_script(
    script: &str,
    request: &Request,
    variables: Option<ScriptVariables>,
) -> Result<(Request, ScriptVariables)> {
    let mut runner = PreRequestRunner::new()?;
    if let Some(vars) = variables {
        runner = runner.with_variables(vars);
    }

    let modified_request = runner.execute(script, request)?;
    let variables = runner.variables().clone();

    Ok((modified_request, variables))
}

/// Run post-request script on a response
pub fn run_post_request_script(
    script: &str,
    response: &Response,
    variables: Option<ScriptVariables>,
) -> Result<ScriptVariables> {
    let mut runner = PostRequestRunner::new()?;
    if let Some(vars) = variables {
        runner = runner.with_variables(vars);
    }

    runner.execute(script, response)?;
    Ok(runner.variables().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::Assertions;
    use crate::http::KeyValList;
    use crate::http::request::{Auth, Method, RequestBody};

    fn create_test_request() -> Request {
        Request {
            description: "Test Request".to_string(),
            method: Method::GET,
            url: "https://api.example.com/users".to_string(),
            headers: KeyValList::new(),
            query_params: KeyValList::new(),
            path_params: KeyValList::new(),
            body: RequestBody::None,
            auth: Auth::None,
            assertions: Assertions::default(),
            pre_request: None,
            post_request: None,
        }
    }

    #[test]
    fn test_pre_request_script_modify_method() {
        let request = create_test_request();
        let script = r#"
            request.method = "POST";
        "#;

        let (modified_request, _) = run_pre_request_script(script, &request, None).unwrap();
        assert_eq!(modified_request.method, Method::POST);
    }

    #[test]
    fn test_pre_request_script_add_headers() {
        let request = create_test_request();
        let script = r#"
            request.headers["Authorization"] = "Bearer token";
            request.headers["X-Custom"] = "value";
        "#;

        let (modified_request, _) = run_pre_request_script(script, &request, None).unwrap();
        assert_eq!(modified_request.headers.iter().count(), 2);
    }

    #[test]
    fn test_pre_request_script_modify_url() {
        let request = create_test_request();
        let script = r#"
            request.url = "https://api.example.com/posts";
        "#;

        let (modified_request, _) = run_pre_request_script(script, &request, None).unwrap();
        assert_eq!(modified_request.url, "https://api.example.com/posts");
    }

    #[test]
    fn test_variables_persist() {
        let request = create_test_request();
        let script = r#"
            variables["token"] = "abc123";
            variables["user"] = "john";
        "#;

        let (_, variables) = run_pre_request_script(script, &request, None).unwrap();
        assert_eq!(variables.get("token"), Some("abc123".to_string()));
        assert_eq!(variables.get("user"), Some("john".to_string()));
    }

    #[test]
    fn test_console_log() {
        let request = create_test_request();
        let script = r#"
            console.log("Test message");
            request.method = "PUT";
        "#;

        let (modified_request, _) = run_pre_request_script(script, &request, None).unwrap();
        assert_eq!(modified_request.method, Method::PUT);
    }
}
