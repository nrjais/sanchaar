pub mod runner;

use crate::http::request::Method;
use crate::http::{KeyValList, KeyValue};
use anyhow::{Context, Result};
use rquickjs::{Ctx, Object, Runtime};
use std::collections::HashMap;
use std::time::Duration;

/// Request context exposed to scripts for modification
#[derive(Debug, Clone)]
pub struct ScriptRequest {
    pub method: Method,
    pub url: String,
    pub headers: KeyValList,
    pub query_params: KeyValList,
    pub body: Option<String>,
}

impl ScriptRequest {
    pub fn new(method: Method, url: String, headers: KeyValList, query_params: KeyValList) -> Self {
        Self {
            method,
            url,
            headers,
            query_params,
            body: None,
        }
    }

    /// Set request method from script
    pub fn set_method(&mut self, method: String) -> Result<()> {
        self.method = method.parse().context("Invalid HTTP method")?;
        Ok(())
    }

    /// Set request URL from script
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    /// Add or update a header from script
    pub fn set_header(&mut self, key: String, value: String) {
        // Remove existing header with same key (case-insensitive)
        self.headers.retain(|h| !h.name.eq_ignore_ascii_case(&key));
        self.headers.push(KeyValue {
            name: key,
            value,
            disabled: false,
        });
    }

    /// Remove a header from script
    pub fn remove_header(&mut self, key: String) {
        self.headers.retain(|h| !h.name.eq_ignore_ascii_case(&key));
    }

    /// Get a header value from script
    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|h| !h.disabled && h.name.eq_ignore_ascii_case(key))
            .map(|h| h.value.clone())
    }

    /// Add or update a query parameter from script
    pub fn set_query_param(&mut self, key: String, value: String) {
        // Remove existing param with same key
        self.query_params.retain(|p| p.name != key);
        self.query_params.push(KeyValue {
            name: key,
            value,
            disabled: false,
        });
    }

    /// Remove a query parameter from script
    pub fn remove_query_param(&mut self, key: String) {
        self.query_params.retain(|p| p.name != key);
    }

    /// Get a query parameter value from script
    pub fn get_query_param(&self, key: &str) -> Option<String> {
        self.query_params
            .iter()
            .find(|p| !p.disabled && p.name == key)
            .map(|p| p.value.clone())
    }

    /// Get all headers as a map (for script access)
    pub fn headers_map(&self) -> HashMap<String, String> {
        self.headers
            .iter()
            .filter(|h| !h.disabled)
            .map(|h| (h.name.clone(), h.value.clone()))
            .collect()
    }

    /// Get all query params as a map (for script access)
    pub fn query_params_map(&self) -> HashMap<String, String> {
        self.query_params
            .iter()
            .filter(|p| !p.disabled)
            .map(|p| (p.name.clone(), p.value.clone()))
            .collect()
    }
}

/// Response context exposed to scripts (for post-request scripts)
#[derive(Debug, Clone)]
pub struct ScriptResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration_ms: u64,
}

/// Variables that can be set/retrieved by scripts
#[derive(Debug, Clone, Default)]
pub struct ScriptVariables {
    variables: HashMap<String, String>,
}

impl ScriptVariables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.variables.get(key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.variables.remove(&key);
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn all(&self) -> HashMap<String, String> {
        self.variables.clone()
    }
}

/// Script execution engine with sandboxed JavaScript environment
pub struct ScriptEngine {
    runtime: Runtime,
    timeout: Duration,
}

impl ScriptEngine {
    pub fn new() -> Result<Self> {
        let runtime = Runtime::new()?;

        // Set memory and execution limits
        runtime.set_memory_limit(50 * 1024 * 1024); // 50MB memory limit
        runtime.set_max_stack_size(256 * 1024); // 256KB stack

        Ok(Self {
            runtime,
            timeout: Duration::from_secs(5), // 5 second timeout
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Execute a pre-request script
    pub fn execute_pre_request(
        &self,
        script: &str,
        request: &mut ScriptRequest,
        variables: &mut ScriptVariables,
    ) -> Result<()> {
        let context = rquickjs::Context::full(&self.runtime)?;

        context.with(|ctx| {
            self.setup_console(&ctx)?;
            self.setup_request_object(&ctx, request)?;
            self.setup_variables_object(&ctx, variables)?;

            // Execute the script
            ctx.eval::<(), _>(script)
                .context("Script execution failed")?;

            // Extract modifications from the request object
            self.extract_request_modifications(&ctx, request)?;
            self.extract_variables(&ctx, variables)?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    /// Execute a post-request script
    pub fn execute_post_request(
        &self,
        script: &str,
        response: &ScriptResponse,
        variables: &mut ScriptVariables,
    ) -> Result<()> {
        let context = rquickjs::Context::full(&self.runtime)?;

        context.with(|ctx| {
            self.setup_console(&ctx)?;
            self.setup_response_object(&ctx, response)?;
            self.setup_variables_object(&ctx, variables)?;

            // Execute the script
            ctx.eval::<(), _>(script)
                .context("Script execution failed")?;

            // Extract variable modifications
            self.extract_variables(&ctx, variables)?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    /// Setup console.log for debugging
    fn setup_console<'js>(&self, ctx: &Ctx<'js>) -> Result<()> {
        let console = Object::new(ctx.clone())?;

        console.set(
            "log",
            rquickjs::Function::new(ctx.clone(), |msg: String| {
                log::info!("[Script] {}", msg);
            })?,
        )?;

        console.set(
            "error",
            rquickjs::Function::new(ctx.clone(), |msg: String| {
                log::error!("[Script] {}", msg);
            })?,
        )?;

        console.set(
            "warn",
            rquickjs::Function::new(ctx.clone(), |msg: String| {
                log::warn!("[Script] {}", msg);
            })?,
        )?;

        ctx.globals().set("console", console)?;
        Ok(())
    }

    /// Setup request object in JavaScript context
    fn setup_request_object<'js>(&self, ctx: &Ctx<'js>, request: &ScriptRequest) -> Result<()> {
        let request_obj = Object::new(ctx.clone())?;

        // Set basic properties
        request_obj.set("method", request.method.to_string())?;
        request_obj.set("url", request.url.clone())?;

        // Set headers as object
        let headers_obj = Object::new(ctx.clone())?;
        for (key, value) in request.headers_map() {
            headers_obj.set(key, value)?;
        }
        request_obj.set("headers", headers_obj)?;

        // Set query params as object
        let params_obj = Object::new(ctx.clone())?;
        for (key, value) in request.query_params_map() {
            params_obj.set(key, value)?;
        }
        request_obj.set("params", params_obj)?;

        ctx.globals().set("request", request_obj)?;
        Ok(())
    }

    /// Setup response object in JavaScript context
    fn setup_response_object<'js>(&self, ctx: &Ctx<'js>, response: &ScriptResponse) -> Result<()> {
        let response_obj = Object::new(ctx.clone())?;

        response_obj.set("status", response.status as i32)?;
        response_obj.set("body", response.body.clone())?;
        response_obj.set("duration", response.duration_ms as i64)?;

        // Set headers as object
        let headers_obj = Object::new(ctx.clone())?;
        for (key, value) in &response.headers {
            headers_obj.set(key.clone(), value.clone())?;
        }
        response_obj.set("headers", headers_obj)?;

        ctx.globals().set("response", response_obj)?;
        Ok(())
    }

    /// Setup variables object in JavaScript context
    fn setup_variables_object<'js>(
        &self,
        ctx: &Ctx<'js>,
        variables: &ScriptVariables,
    ) -> Result<()> {
        let vars_obj = Object::new(ctx.clone())?;

        for (key, value) in variables.all() {
            vars_obj.set(key, value)?;
        }

        ctx.globals().set("variables", vars_obj)?;
        Ok(())
    }

    /// Extract request modifications from JavaScript context
    fn extract_request_modifications<'js>(
        &self,
        ctx: &Ctx<'js>,
        request: &mut ScriptRequest,
    ) -> Result<()> {
        let globals = ctx.globals();
        let request_obj: Object = globals.get("request")?;

        // Extract method
        if let Ok(method) = request_obj.get::<_, String>("method")
            && let Err(e) = request.set_method(method)
        {
            log::warn!("Failed to set method from script: {}", e);
        }

        // Extract URL
        if let Ok(url) = request_obj.get::<_, String>("url") {
            request.set_url(url);
        }

        // Extract headers - only update modified headers, don't clear existing ones
        if let Ok(headers_obj) = request_obj.get::<_, Object>("headers") {
            for key in headers_obj.keys::<String>() {
                if let Ok(key) = key
                    && let Ok(value) = headers_obj.get::<_, String>(&key)
                {
                    request.set_header(key, value);
                }
            }
        }

        // Extract query params - only update modified params, don't clear existing ones
        if let Ok(params_obj) = request_obj.get::<_, Object>("params") {
            for key in params_obj.keys::<String>() {
                if let Ok(key) = key
                    && let Ok(value) = params_obj.get::<_, String>(&key)
                {
                    request.set_query_param(key, value);
                }
            }
        }

        Ok(())
    }

    /// Extract variables from JavaScript context
    fn extract_variables<'js>(
        &self,
        ctx: &Ctx<'js>,
        variables: &mut ScriptVariables,
    ) -> Result<()> {
        let globals = ctx.globals();
        let vars_obj: Object = globals.get("variables")?;

        variables.clear();
        for key in vars_obj.keys::<String>() {
            if let Ok(key) = key
                && let Ok(value) = vars_obj.get::<_, String>(&key)
            {
                variables.set(key, value);
            }
        }

        Ok(())
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create script engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_method() {
        let mut request = ScriptRequest::new(
            Method::GET,
            "https://api.example.com".to_string(),
            KeyValList::new(),
            KeyValList::new(),
        );
        let mut variables = ScriptVariables::new();

        let script = r#"
            request.method = "POST";
        "#;

        let engine = ScriptEngine::new().unwrap();
        engine
            .execute_pre_request(script, &mut request, &mut variables)
            .unwrap();

        assert_eq!(request.method, Method::POST);
    }

    #[test]
    fn test_modify_headers() {
        let mut request = ScriptRequest::new(
            Method::GET,
            "https://api.example.com".to_string(),
            KeyValList::new(),
            KeyValList::new(),
        );
        let mut variables = ScriptVariables::new();

        let script = r#"
            request.headers["Authorization"] = "Bearer token123";
            request.headers["Content-Type"] = "application/json";
        "#;

        let engine = ScriptEngine::new().unwrap();
        engine
            .execute_pre_request(script, &mut request, &mut variables)
            .unwrap();

        assert_eq!(
            request.get_header("Authorization"),
            Some("Bearer token123".to_string())
        );
        assert_eq!(
            request.get_header("Content-Type"),
            Some("application/json".to_string())
        );
    }

    #[test]
    fn test_modify_query_params() {
        let mut request = ScriptRequest::new(
            Method::GET,
            "https://api.example.com".to_string(),
            KeyValList::new(),
            KeyValList::new(),
        );
        let mut variables = ScriptVariables::new();

        let script = r#"
            request.params["api_key"] = "secret123";
            request.params["limit"] = "10";
        "#;

        let engine = ScriptEngine::new().unwrap();
        engine
            .execute_pre_request(script, &mut request, &mut variables)
            .unwrap();

        assert_eq!(
            request.get_query_param("api_key"),
            Some("secret123".to_string())
        );
        assert_eq!(request.get_query_param("limit"), Some("10".to_string()));
    }

    #[test]
    fn test_variables() {
        let mut request = ScriptRequest::new(
            Method::GET,
            "https://api.example.com".to_string(),
            KeyValList::new(),
            KeyValList::new(),
        );
        let mut variables = ScriptVariables::new();

        let script = r#"
            variables["token"] = "abc123";
            variables["user_id"] = "42";
            request.headers["Authorization"] = "Bearer " + variables["token"];
        "#;

        let engine = ScriptEngine::new().unwrap();
        engine
            .execute_pre_request(script, &mut request, &mut variables)
            .unwrap();

        assert_eq!(variables.get("token"), Some("abc123".to_string()));
        assert_eq!(variables.get("user_id"), Some("42".to_string()));
        assert_eq!(
            request.get_header("Authorization"),
            Some("Bearer abc123".to_string())
        );
    }

    #[test]
    fn test_console_log() {
        let mut request = ScriptRequest::new(
            Method::GET,
            "https://api.example.com".to_string(),
            KeyValList::new(),
            KeyValList::new(),
        );
        let mut variables = ScriptVariables::new();

        let script = r#"
            console.log("Testing console.log");
            request.method = "POST";
        "#;

        let engine = ScriptEngine::new().unwrap();
        engine
            .execute_pre_request(script, &mut request, &mut variables)
            .unwrap();

        assert_eq!(request.method, Method::POST);
    }
}
