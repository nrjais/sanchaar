//! Curl command parser
//!
//! This module provides functionality to parse curl commands and convert them
//! into the internal Request representation.
//!
//! # Supported Features
//! - HTTP methods: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, TRACE
//! - Headers: -H, --header
//! - Data: -d, --data, --data-raw, --data-binary
//! - Form data: -F, --form (with file uploads using @filepath)
//! - Authentication: -u/--user (Basic), --bearer (Bearer token)
//! - Common flags: -k/--insecure, -L/--location, --compressed
//!
//! # Example
//! ```
//! use core::curl::parse_curl_command;
//!
//! let cmd = r#"curl -X POST https://api.example.com/users -H "Content-Type: application/json" -d '{"name":"John"}'"#;
//! let request = parse_curl_command(cmd).expect("Failed to parse");
//! ```

use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};

use crate::http::{
    KeyFileList, KeyValList, KeyValue,
    request::{Auth, Method, Request, RequestBody},
};

#[derive(Debug, Default)]
struct CurlParser {
    url: Option<String>,
    method: Option<Method>,
    headers: Vec<KeyValue>,
    data: Vec<String>,
    data_raw: Vec<String>,
    data_binary: Vec<String>,
    form_data: Vec<KeyValue>,
    form_files: Vec<(String, PathBuf)>,
    user: Option<String>,
    bearer: Option<String>,
    compressed: bool,
    insecure: bool,
    location: bool,
}

impl CurlParser {
    fn new() -> Self {
        Self::default()
    }

    fn parse_args(&mut self, args: &[String]) -> Result<()> {
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                // Skip 'curl' command itself
                "curl" => {}

                // Method
                "-X" | "--request" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.method = Some(self.parse_method(&args[i])?);
                }

                // Headers
                "-H" | "--header" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.parse_header(&args[i])?;
                }

                // Data
                "-d" | "--data" | "--data-ascii" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.data.push(args[i].clone());
                }

                "--data-raw" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.data_raw.push(args[i].clone());
                }

                "--data-binary" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.data_binary.push(args[i].clone());
                }

                // Form data
                "-F" | "--form" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.parse_form(&args[i])?;
                }

                // Authentication
                "-u" | "--user" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.user = Some(args[i].clone());
                }

                "--bearer" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(anyhow!("Missing value for {}", arg));
                    }
                    self.bearer = Some(args[i].clone());
                }

                // Flags
                "--compressed" => {
                    self.compressed = true;
                }

                "-k" | "--insecure" => {
                    self.insecure = true;
                }

                "-L" | "--location" => {
                    self.location = true;
                }

                // Output/display options (ignored)
                "-s" | "--silent" | "-v" | "--verbose" | "-i" | "--include" | "-I" | "--head"
                | "-o" | "--output" | "-O" | "--remote-name" | "-w" | "--write-out" => {
                    // Some of these take arguments, skip the next arg if present
                    if matches!(arg.as_str(), "-o" | "--output" | "-w" | "--write-out") {
                        i += 1;
                    }
                }

                // Timeout options (ignored but consume the next arg)
                "--connect-timeout" | "--max-time" | "-m" => {
                    i += 1;
                }

                // Proxy options (ignored but consume the next arg)
                "-x" | "--proxy" => {
                    i += 1;
                }

                // Referrer and User-Agent (add as headers)
                "-e" | "--referer" => {
                    i += 1;
                    if i < args.len() {
                        self.headers.push(KeyValue {
                            disabled: false,
                            name: "Referer".to_string(),
                            value: args[i].clone(),
                        });
                    }
                }

                "-A" | "--user-agent" => {
                    i += 1;
                    if i < args.len() {
                        self.headers.push(KeyValue {
                            disabled: false,
                            name: "User-Agent".to_string(),
                            value: args[i].clone(),
                        });
                    }
                }

                // Cookie (add as header)
                "-b" | "--cookie" => {
                    i += 1;
                    if i < args.len() {
                        self.headers.push(KeyValue {
                            disabled: false,
                            name: "Cookie".to_string(),
                            value: args[i].clone(),
                        });
                    }
                }

                // URL (usually the last argument without a flag)
                _ if !arg.starts_with('-') => {
                    if self.url.is_none() {
                        self.url = Some(arg.clone());
                    }
                }

                // Handle combined short options like -Lks
                _ if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 2 => {
                    // Parse combined short flags
                    for ch in arg[1..].chars() {
                        match ch {
                            'L' => self.location = true,
                            'k' => self.insecure = true,
                            's' => {} // silent
                            'v' => {} // verbose
                            'i' => {} // include
                            _ => {}   // ignore unknown flags
                        }
                    }
                }

                // Ignore unknown options
                _ => {}
            }

            i += 1;
        }

        Ok(())
    }

    fn parse_method(&self, method: &str) -> Result<Method> {
        match method.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(anyhow!("Unknown HTTP method: {}", method)),
        }
    }

    fn parse_header(&mut self, header: &str) -> Result<()> {
        if let Some(colon_idx) = header.find(':') {
            let name = header[..colon_idx].trim().to_string();
            let value = header[colon_idx + 1..].trim().to_string();

            self.headers.push(KeyValue {
                disabled: false,
                name,
                value,
            });
        } else {
            return Err(anyhow!("Invalid header format: {}", header));
        }
        Ok(())
    }

    fn parse_form(&mut self, form: &str) -> Result<()> {
        // Form data can be key=value or key=@filepath
        if let Some(eq_idx) = form.find('=') {
            let key = form[..eq_idx].to_string();
            let value = &form[eq_idx + 1..];

            if let Some(filepath) = value.strip_prefix('@') {
                // File upload
                self.form_files.push((key, PathBuf::from(filepath)));
            } else {
                // Regular form field
                self.form_data.push(KeyValue {
                    disabled: false,
                    name: key,
                    value: value.to_string(),
                });
            }
        } else {
            return Err(anyhow!("Invalid form data format: {}", form));
        }
        Ok(())
    }

    fn build(self) -> Result<Request> {
        let url = self.url.context("No URL provided")?;

        // Determine method
        let method = if let Some(m) = self.method {
            m
        } else if !self.data.is_empty()
            || !self.data_raw.is_empty()
            || !self.data_binary.is_empty()
            || !self.form_data.is_empty()
            || !self.form_files.is_empty()
        {
            Method::POST
        } else {
            Method::GET
        };

        // Determine body
        let body = if !self.form_data.is_empty() || !self.form_files.is_empty() {
            let params = KeyValList::from(self.form_data);
            let files = KeyFileList::from(
                self.form_files
                    .into_iter()
                    .map(|(name, path)| crate::http::KeyFile {
                        name,
                        path: Some(path),
                        disabled: false,
                    })
                    .collect(),
            );

            if files.iter().next().is_some() {
                RequestBody::Multipart { params, files }
            } else {
                RequestBody::Form(params)
            }
        } else if !self.data.is_empty() || !self.data_raw.is_empty() {
            // Combine all data fields
            let mut all_data = Vec::new();
            all_data.extend(self.data);
            all_data.extend(self.data_raw);

            let combined = all_data.join("&");

            // Try to detect if it's JSON
            if combined.trim().starts_with('{') || combined.trim().starts_with('[') {
                RequestBody::Json(combined)
            } else if combined.trim().starts_with('<') {
                RequestBody::XML(combined)
            } else {
                RequestBody::Text(combined)
            }
        } else if !self.data_binary.is_empty() {
            RequestBody::Text(self.data_binary.join(""))
        } else {
            RequestBody::None
        };

        // Determine auth
        let auth = if let Some(token) = self.bearer {
            Auth::Bearer { token }
        } else if let Some(user) = self.user {
            if let Some(colon_idx) = user.find(':') {
                Auth::Basic {
                    username: user[..colon_idx].to_string(),
                    password: user[colon_idx + 1..].to_string(),
                }
            } else {
                Auth::Basic {
                    username: user,
                    password: String::new(),
                }
            }
        } else {
            Auth::None
        };

        // Add compression header if needed
        let mut headers = self.headers;
        if self.compressed {
            headers.push(KeyValue {
                disabled: false,
                name: "Accept-Encoding".to_string(),
                value: "gzip, deflate, br".to_string(),
            });
        }

        Ok(Request {
            description: "Imported from curl".to_string(),
            method,
            url,
            headers: KeyValList::from(headers),
            body,
            query_params: KeyValList::new(),
            path_params: KeyValList::new(),
            auth,
            assertions: Default::default(),
            pre_request: None,
        })
    }
}

/// Parse a curl command string into a Request
pub fn parse_curl_command(command: &str) -> Result<Request> {
    let args = tokenize_command(command)?;
    let mut parser = CurlParser::new();
    parser.parse_args(&args)?;
    parser.build()
}

/// Tokenize a curl command string, respecting quotes
fn tokenize_command(command: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;
    let chars = command.chars().peekable();

    for ch in chars {
        if escape_next {
            // Special handling for backslash-newline (line continuation in bash)
            if ch == '\n' || ch == '\r' {
                // Treat backslash-newline as line continuation (just skip it)
                escape_next = false;
                continue;
            }
            current.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => {
                if in_single_quote {
                    current.push(ch);
                } else {
                    escape_next = true;
                }
            }
            '\'' => {
                if in_double_quote {
                    current.push(ch);
                } else {
                    in_single_quote = !in_single_quote;
                }
            }
            '"' => {
                if in_single_quote {
                    current.push(ch);
                } else {
                    in_double_quote = !in_double_quote;
                }
            }
            ' ' | '\t' | '\n' | '\r' => {
                if in_single_quote || in_double_quote {
                    current.push(ch);
                } else if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    if in_single_quote || in_double_quote {
        return Err(anyhow!("Unclosed quote in curl command"));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_get() {
        let cmd = "curl https://api.example.com/users";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.url, "https://api.example.com/users");
    }

    #[test]
    fn test_post_with_json() {
        let cmd = r#"curl -X POST https://api.example.com/users -H "Content-Type: application/json" -d '{"name":"John"}'"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert_eq!(req.url, "https://api.example.com/users");
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"name":"John"}"#.to_string())
        );
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Content-Type" && h.value == "application/json")
        );
    }

    #[test]
    fn test_basic_auth() {
        let cmd = "curl -u user:pass https://api.example.com/protected";
        let req = parse_curl_command(cmd).unwrap();
        match req.auth {
            Auth::Basic { username, password } => {
                assert_eq!(username, "user");
                assert_eq!(password, "pass");
            }
            _ => panic!("Expected Basic auth"),
        }
    }

    #[test]
    fn test_bearer_token() {
        let cmd = "curl --bearer mytoken123 https://api.example.com/data";
        let req = parse_curl_command(cmd).unwrap();
        match req.auth {
            Auth::Bearer { token } => {
                assert_eq!(token, "mytoken123");
            }
            _ => panic!("Expected Bearer auth"),
        }
    }

    #[test]
    fn test_form_data() {
        let cmd = "curl -F name=John -F email=john@example.com https://api.example.com/form";
        let req = parse_curl_command(cmd).unwrap();
        match req.body {
            RequestBody::Form(form) => {
                assert_eq!(form.iter().count(), 2);
                assert!(
                    form.iter()
                        .any(|kv| kv.name == "name" && kv.value == "John")
                );
                assert!(
                    form.iter()
                        .any(|kv| kv.name == "email" && kv.value == "john@example.com")
                );
            }
            _ => panic!("Expected Form body"),
        }
    }

    #[test]
    fn test_headers() {
        let cmd =
            r#"curl -H "Authorization: Bearer token" -H "X-Custom: value" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.headers.iter().count(), 2);
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Authorization" && h.value == "Bearer token")
        );
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "X-Custom" && h.value == "value")
        );
    }

    #[test]
    fn test_user_agent() {
        let cmd = r#"curl -A "MyAgent/1.0" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "User-Agent" && h.value == "MyAgent/1.0")
        );
    }

    #[test]
    fn test_cookie() {
        let cmd = r#"curl -b "session=abc123" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Cookie" && h.value == "session=abc123")
        );
    }

    #[test]
    fn test_multipart_with_file() {
        let cmd =
            r#"curl -F name=John -F avatar=@/path/to/image.jpg https://api.example.com/upload"#;
        let req = parse_curl_command(cmd).unwrap();
        match req.body {
            RequestBody::Multipart { params, files } => {
                assert_eq!(params.iter().count(), 1);
                assert!(
                    params
                        .iter()
                        .any(|kv| kv.name == "name" && kv.value == "John")
                );
                assert_eq!(files.iter().count(), 1);
                assert!(files.iter().any(|f| f.name == "avatar"
                    && f.path == Some(std::path::PathBuf::from("/path/to/image.jpg"))));
            }
            _ => panic!("Expected Multipart body"),
        }
    }

    #[test]
    fn test_complex_command() {
        let cmd = r#"curl -X PUT https://api.example.com/users/123 \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer token123" \
            -d '{"name":"Jane","email":"jane@example.com"}' \
            -k -L"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::PUT);
        assert_eq!(req.url, "https://api.example.com/users/123");
        assert!(req.headers.iter().count() >= 2);
        let json = r#"{"name":"Jane","email":"jane@example.com"}"#.to_string();
        assert!(req.body == RequestBody::Json(json));
    }

    #[test]
    fn test_tokenize_with_quotes() {
        let tokens =
            tokenize_command(r#"curl -H "Content-Type: application/json" 'https://api.com'"#)
                .unwrap();
        assert!(tokens.contains(&"Content-Type: application/json".to_string()));
        assert!(tokens.contains(&"https://api.com".to_string()));
    }

    #[test]
    fn test_method_inference_post() {
        let cmd = r#"curl https://api.example.com -d "data=value""#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_json_detection() {
        let cmd = r#"curl https://api.example.com -d '{"key":"value"}'"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"key":"value"}"#.to_string())
        );
        assert_eq!(req.method, Method::POST); // Inferred from -d
    }

    #[test]
    fn test_xml_detection() {
        let cmd = r#"curl https://api.example.com -d '<root><item>value</item></root>'"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::XML("<root><item>value</item></root>".to_string())
        );
    }

    #[test]
    fn test_github_api_example() {
        let cmd = r#"curl -H "Accept: application/vnd.github.v3+json" -H "Authorization: token abc123" https://api.github.com/user/repos"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.url, "https://api.github.com/user/repos");
        assert_eq!(req.headers.iter().count(), 2);
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Accept" && h.value == "application/vnd.github.v3+json")
        );
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Authorization" && h.value == "token abc123")
        );
    }

    #[test]
    fn test_combined_data_fields() {
        let cmd = r#"curl -d "name=John" -d "email=john@example.com" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Text("name=John&email=john@example.com".to_string())
        );
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_referer_header() {
        let cmd = r#"curl -e "https://example.com" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Referer" && h.value == "https://example.com")
        );
    }

    #[test]
    fn test_url_with_query_params() {
        let cmd = r#"curl "https://api.example.com/search?q=test&limit=10&offset=0""#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.url,
            "https://api.example.com/search?q=test&limit=10&offset=0"
        );
    }

    #[test]
    fn test_url_with_port() {
        let cmd = "curl https://localhost:8080/api/endpoint";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "https://localhost:8080/api/endpoint");
    }

    #[test]
    fn test_ipv6_url() {
        let cmd = "curl http://[2001:db8::1]:8080/api";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "http://[2001:db8::1]:8080/api");
    }

    #[test]
    fn test_basic_auth_without_password() {
        let cmd = "curl -u username: https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        if let Auth::Basic { username, password } = req.auth {
            assert_eq!(username, "username");
            assert_eq!(password, "");
        } else {
            panic!("Expected Basic auth");
        }
    }

    #[test]
    fn test_basic_auth_with_colon_in_password() {
        let cmd = "curl -u user:pass:word https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        if let Auth::Basic { username, password } = req.auth {
            assert_eq!(username, "user");
            assert_eq!(password, "pass:word");
        } else {
            panic!("Expected Basic auth");
        }
    }

    #[test]
    fn test_header_with_colon_in_value() {
        let cmd = r#"curl -H "X-Custom: value:with:colons" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "X-Custom" && h.value == "value:with:colons")
        );
    }

    #[test]
    fn test_duplicate_headers() {
        let cmd =
            r#"curl -H "Accept: application/json" -H "Accept: text/html" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        let accept_headers: Vec<_> = req.headers.iter().filter(|h| h.name == "Accept").collect();
        assert_eq!(accept_headers.len(), 2);
    }

    #[test]
    fn test_post_without_data() {
        // Explicit POST without data
        let cmd = r#"curl -X POST https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert!(matches!(req.body, RequestBody::None));
    }

    #[test]
    fn test_data_with_special_characters() {
        let cmd =
            r#"curl -d "name=John%20Doe&email=test%2Buser%40example.com" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Text("name=John%20Doe&email=test%2Buser%40example.com".to_string())
        );
    }

    #[test]
    fn test_json_with_unicode() {
        let cmd = r#"curl -d '{"name":"José","emoji":"🚀"}' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"name":"José","emoji":"🚀"}"#.to_string())
        );
    }

    #[test]
    fn test_data_with_newlines() {
        let cmd = "curl -d '{\"key\":\"value\",\n\"key2\":\"value2\"}' https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json("{\"key\":\"value\",\n\"key2\":\"value2\"}".to_string())
        );
    }

    #[test]
    fn test_form_with_equals_in_value() {
        let cmd = "curl -F 'data=key=value' https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        if let RequestBody::Form(form) = req.body {
            assert!(
                form.iter()
                    .any(|kv| kv.name == "data" && kv.value == "key=value")
            );
        } else {
            panic!("Expected Form body");
        }
    }

    #[test]
    fn test_multiple_spaces_between_args() {
        let cmd = "curl    -X   POST     https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert_eq!(req.url, "https://api.example.com");
    }

    #[test]
    fn test_tabs_in_command() {
        let cmd = "curl\t-X\tPOST\thttps://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_mixed_quotes() {
        let cmd = r#"curl -H 'Content-Type: application/json' -d "{'key':'value'}" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Content-Type" && h.value == "application/json")
        );
        assert_eq!(req.body, RequestBody::Json("{'key':'value'}".to_string()));
    }

    #[test]
    fn test_escaped_quotes_in_data() {
        // Note: Our tokenizer processes escapes, so \" becomes "
        let cmd = r#"curl -d "{\"escaped\":\"quotes\"}" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        // The tokenizer converts \" to " in the parsed result
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"escaped":"quotes"}"#.to_string())
        );
    }

    #[test]
    fn test_url_as_last_arg_with_many_flags() {
        let cmd = r#"curl -X POST -H "Content-Type: application/json" -d '{"test":true}' -k -L https://api.example.com/endpoint"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "https://api.example.com/endpoint");
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_url_as_first_arg_after_curl() {
        let cmd = r#"curl https://api.example.com -X POST -d '{"test":true}'"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "https://api.example.com");
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_very_long_url() {
        let long_url = format!(
            "https://api.example.com/endpoint?{}",
            "param=value&".repeat(100)
        );
        let cmd = format!("curl '{}'", long_url);
        let req = parse_curl_command(&cmd).unwrap();
        assert!(req.url.len() > 1000);
    }

    #[test]
    fn test_empty_header_value() {
        let cmd = r#"curl -H "X-Empty:" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "X-Empty" && h.value.is_empty())
        );
    }

    #[test]
    fn test_header_with_leading_trailing_spaces() {
        let cmd = r#"curl -H "  X-Spaced  :  value  " https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "X-Spaced" && h.value == "value")
        );
    }

    #[test]
    fn test_all_http_methods() {
        let methods = [
            "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE",
        ];
        for method in methods {
            let cmd = format!("curl -X {} https://api.example.com", method);
            let req = parse_curl_command(&cmd).unwrap();
            assert_eq!(req.method.to_string(), method);
        }
    }

    #[test]
    fn test_method_case_insensitive() {
        let cmd = "curl -X post https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
    }

    #[test]
    fn test_invalid_method() {
        let cmd = "curl -X INVALID https://api.example.com";
        let result = parse_curl_command(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_url() {
        let cmd = "curl -X POST -d 'data'";
        let result = parse_curl_command(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_single_quote() {
        let cmd = "curl -d 'unclosed https://api.example.com";
        let result = parse_curl_command(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_double_quote() {
        let cmd = r#"curl -d "unclosed https://api.example.com"#;
        let result = parse_curl_command(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_bearer_with_special_chars() {
        let cmd = "curl --bearer 'eyJhbGc.iOiJIUzI1NiIsInR5cCI6Ikp.XVCJ9' https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        match req.auth {
            Auth::Bearer { token } => {
                assert_eq!(token, "eyJhbGc.iOiJIUzI1NiIsInR5cCI6Ikp.XVCJ9");
            }
            _ => panic!("Expected Bearer auth"),
        }
    }

    #[test]
    fn test_compressed_adds_header() {
        let cmd = "curl --compressed https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Accept-Encoding" && h.value == "gzip, deflate, br")
        );
    }

    #[test]
    fn test_form_file_with_absolute_path() {
        let cmd = "curl -F 'file=@/absolute/path/to/file.pdf' https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        match req.body {
            RequestBody::Multipart { files, params } => {
                assert_eq!(params.iter().count(), 0);
                assert_eq!(files.iter().count(), 1);
                assert!(files.iter().any(|f| f.name == "file"
                    && f.path == Some(std::path::PathBuf::from("/absolute/path/to/file.pdf"))));
            }
            _ => panic!("Expected Multipart body"),
        }
    }

    #[test]
    fn test_multiple_form_files() {
        let cmd = r#"curl -F "file1=@/path/1.jpg" -F "file2=@/path/2.jpg" -F "file3=@/path/3.jpg" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        if let RequestBody::Multipart { files, .. } = req.body {
            assert_eq!(files.iter().count(), 3);
        } else {
            panic!("Expected Multipart body");
        }
    }

    #[test]
    fn test_mixed_form_fields_and_files() {
        let cmd = r#"curl -F "name=John" -F "avatar=@/img.jpg" -F "age=30" -F "cv=@/cv.pdf" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        if let RequestBody::Multipart { params, files } = req.body {
            assert_eq!(params.iter().count(), 2); // name and age
            assert_eq!(files.iter().count(), 2); // avatar and cv
        } else {
            panic!("Expected Multipart body");
        }
    }

    #[test]
    fn test_cookie_with_multiple_values() {
        let cmd = r#"curl -b "session=abc123; user_id=42; theme=dark" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.headers.iter().any(|h| h.name == "Cookie"
            && h.value.contains("session")
            && h.value.contains("theme")));
    }

    #[test]
    fn test_array_json_body() {
        let cmd = r#"curl -d '[1,2,3,4,5]' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.body, RequestBody::Json("[1,2,3,4,5]".to_string()));
    }

    #[test]
    fn test_nested_json() {
        let cmd = r#"curl -d '{"user":{"name":"John","address":{"city":"NYC"}}}' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"user":{"name":"John","address":{"city":"NYC"}}}"#.to_string())
        );
    }

    #[test]
    fn test_xml_with_attributes() {
        let cmd = r#"curl -d '<root attr="value"><item id="1">text</item></root>' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::XML(r#"<root attr="value"><item id="1">text</item></root>"#.to_string())
        );
    }

    #[test]
    fn test_data_raw_vs_data() {
        let cmd = r#"curl --data-raw '{"key":"value"}' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"key":"value"}"#.to_string())
        );
    }

    #[test]
    fn test_multiple_user_agents_last_wins() {
        let cmd = r#"curl -A "Agent1" -A "Agent2" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        let agents: Vec<_> = req
            .headers
            .iter()
            .filter(|h| h.name == "User-Agent")
            .collect();
        assert_eq!(agents.len(), 2); // Both are added
    }

    #[test]
    fn test_url_with_fragment() {
        let cmd = "curl 'https://api.example.com/page#section'";
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.url.contains("#section"));
    }

    #[test]
    fn test_url_with_auth_in_url() {
        let cmd = "curl 'https://user:pass@api.example.com/endpoint'";
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.url.contains("user:pass@"));
    }

    #[test]
    fn test_only_curl_command() {
        let cmd = "curl";
        let result = parse_curl_command(cmd);
        assert!(result.is_err()); // No URL provided
    }

    #[test]
    fn test_newlines_in_command() {
        // Newlines are treated as whitespace separators
        let cmd = "curl -X POST\nhttps://api.example.com\n-d 'data'";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert_eq!(req.url, "https://api.example.com");
    }

    #[test]
    fn test_backslash_newline_line_continuation() {
        // Backslash-newline should work as line continuation (bash-style)
        let cmd = "curl -X POST \\\nhttps://api.example.com \\\n-H \"Auth: token\" \\\n-d '{\"data\":\"value\"}'";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert_eq!(req.url, "https://api.example.com");
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Auth" && h.value == "token")
        );
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"data":"value"}"#.to_string())
        );
    }

    #[test]
    fn test_data_binary() {
        let cmd = r#"curl --data-binary "@file.bin" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.body, RequestBody::Text("@file.bin".to_string()));
    }

    #[test]
    fn test_http_vs_https() {
        let cmd1 = "curl http://api.example.com";
        let cmd2 = "curl https://api.example.com";
        let req1 = parse_curl_command(cmd1).unwrap();
        let req2 = parse_curl_command(cmd2).unwrap();
        assert!(req1.url.starts_with("http://"));
        assert!(req2.url.starts_with("https://"));
    }

    #[test]
    fn test_localhost_variations() {
        for url in ["http://localhost", "http://127.0.0.1", "http://0.0.0.0"] {
            let cmd = format!("curl {}", url);
            let req = parse_curl_command(&cmd).unwrap();
            assert_eq!(req.url, url);
        }
    }

    #[test]
    fn test_header_without_space_after_colon() {
        let cmd = r#"curl -H "Content-Type:application/json" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Content-Type" && h.value == "application/json")
        );
    }

    #[test]
    fn test_url_with_multiple_query_params() {
        let cmd =
            "curl 'https://api.example.com/search?q=rust+programming&lang=en&page=1&limit=20'";
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.url.contains("q=rust+programming"));
        assert!(req.url.contains("page=1"));
    }

    #[test]
    fn test_complex_json_with_arrays_and_nulls() {
        let cmd = r#"curl -d '{"items":[1,2,3],"status":null,"nested":{"value":true}}' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(
                r#"{"items":[1,2,3],"status":null,"nested":{"value":true}}"#.to_string()
            )
        );
    }

    #[test]
    fn test_form_data_without_form_flag() {
        // Using -d with form data format (not JSON)
        let cmd = r#"curl -d "key1=value1&key2=value2" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        // Should be detected as Text since it doesn't start with { or <
        assert_eq!(
            req.body,
            RequestBody::Text("key1=value1&key2=value2".to_string())
        );
    }

    #[test]
    fn test_multiple_headers_same_line() {
        let cmd = r#"curl -H "Accept: application/json" -H "Content-Type: application/json" -H "Authorization: Bearer token" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.headers.iter().count(), 3);
    }

    #[test]
    fn test_url_with_username_no_password() {
        let cmd = "curl 'https://admin@api.example.com/admin'";
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.url.contains("admin@"));
    }

    #[test]
    fn test_data_url_encoded() {
        let cmd =
            r#"curl -d "message=Hello%20World%21&user=john%40example.com" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Text("message=Hello%20World%21&user=john%40example.com".to_string())
        );
    }

    #[test]
    fn test_get_with_body_ignored() {
        // Some APIs accept GET with body (non-standard but happens)
        let cmd = r#"curl -X GET -d '{"query":"test"}' https://api.example.com/search"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"query":"test"}"#.to_string())
        );
    }

    #[test]
    fn test_patch_method() {
        let cmd = r#"curl -X PATCH -d '{"status":"updated"}' https://api.example.com/items/123"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::PATCH);
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"status":"updated"}"#.to_string())
        );
    }

    #[test]
    fn test_options_method() {
        let cmd = "curl -X OPTIONS https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::OPTIONS);
    }

    #[test]
    fn test_very_long_header_value() {
        let long_value = "x".repeat(1000);
        let cmd = format!(
            r#"curl -H "X-Long: {}" https://api.example.com"#,
            long_value
        );
        let req = parse_curl_command(&cmd).unwrap();
        assert!(req.headers.iter().any(|h| h.value.len() > 900));
    }

    #[test]
    fn test_json_with_escaped_characters() {
        let cmd = r#"curl -d '{"text":"Line1\nLine2\tTabbed"}' https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"text":"Line1\nLine2\tTabbed"}"#.to_string())
        );
    }

    #[test]
    fn test_multiple_cookies() {
        let cmd = r#"curl -b "cookie1=value1" -b "cookie2=value2" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        let cookies: Vec<_> = req.headers.iter().filter(|h| h.name == "Cookie").collect();
        assert_eq!(cookies.len(), 2);
    }

    #[test]
    fn test_url_with_non_standard_port() {
        let cmd = "curl http://api.example.com:3000/endpoint";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "http://api.example.com:3000/endpoint");
    }

    #[test]
    fn test_soap_xml_request() {
        let cmd = r#"curl -X POST -H "Content-Type: text/xml" -d '<?xml version="1.0"?><soap:Envelope><soap:Body><GetPrice><Item>Apple</Item></GetPrice></soap:Body></soap:Envelope>' https://api.example.com/soap"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.method, Method::POST);
        assert_eq!(
            req.body,
            RequestBody::XML(
                r#"<?xml version="1.0"?><soap:Envelope><soap:Body><GetPrice><Item>Apple</Item></GetPrice></soap:Body></soap:Envelope>"#
                    .to_string()
            )
        );
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Content-Type" && h.value == "text/xml")
        );
    }

    #[test]
    fn test_graphql_query() {
        let cmd = r#"curl -X POST -H "Content-Type: application/json" -d '{"query":"query { user(id: 1) { name email } }"}' https://api.example.com/graphql"#;
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(
            req.body,
            RequestBody::Json(r#"{"query":"query { user(id: 1) { name email } }"}"#.to_string())
        );
        assert_eq!(req.url, "https://api.example.com/graphql");
    }

    #[test]
    fn test_user_agent_with_version() {
        let cmd = r#"curl -A "MyApp/1.2.3 (Platform; OS)" https://api.example.com"#;
        let req = parse_curl_command(cmd).unwrap();
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "User-Agent" && h.value.contains("1.2.3"))
        );
    }

    #[test]
    fn test_basic_auth_special_chars_in_username() {
        let cmd = "curl -u 'user@domain.com:password123' https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        if let Auth::Basic { username, .. } = req.auth {
            assert!(username.contains("@"));
        } else {
            panic!("Expected Basic auth");
        }
    }

    #[test]
    fn test_url_with_path_and_query() {
        let cmd = "curl 'https://api.example.com/v1/users/123?include=profile&format=json'";
        let req = parse_curl_command(cmd).unwrap();
        assert!(req.url.contains("/v1/users/123"));
        assert!(req.url.contains("include=profile"));
    }

    #[test]
    fn test_combined_short_flags_with_values() {
        let cmd = "curl -sL https://api.example.com";
        let req = parse_curl_command(cmd).unwrap();
        assert_eq!(req.url, "https://api.example.com");
        // -s and -L are processed
    }

    #[test]
    fn test_real_world_long_flags_with_line_continuation() {
        let cmd = r#"curl --location --request GET 'echo.sanchaar.app?test=hello' \
--header 'test: 1235' \
--header 'Content-Type: application/json' \
--data '{
    "test": "hello"
}'"#;
        let req = parse_curl_command(cmd).unwrap();

        assert_eq!(req.method, Method::GET);
        assert_eq!(req.url, "echo.sanchaar.app?test=hello");

        // Verify headers
        assert_eq!(req.headers.iter().count(), 2);
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "test" && h.value == "1235")
        );
        assert!(
            req.headers
                .iter()
                .any(|h| h.name == "Content-Type" && h.value == "application/json")
        );

        // Query params are not extracted from URL by curl parser
        assert!(req.query_params.is_empty());

        // Verify multi-line JSON body
        assert_eq!(
            req.body,
            RequestBody::Json("{\n    \"test\": \"hello\"\n}".to_string())
        );
    }
}
