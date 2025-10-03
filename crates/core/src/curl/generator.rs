//! Curl command generator
//!
//! This module provides functionality to generate curl commands from the internal Request representation.
//!
//! # Supported Features
//! - HTTP methods: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, TRACE
//! - Headers: -H, --header
//! - Data: -d, --data (for JSON, XML, Text bodies)
//! - Form data: -F, --form (with file uploads using @filepath)
//! - Authentication: -u/--user (Basic), --bearer (Bearer token)
//! - Query parameters: merged into URL
//! - Path parameters: substituted in URL
//! - Shell escaping: proper quoting for bash/sh compatibility
//!
//! # Example
//! ```
//! use core::curl::generate_curl_command;
//! use core::http::request::Request;
//!
//! let request = Request::default();
//! let curl_cmd = generate_curl_command(&request);
//! ```

use crate::http::{
    KeyValList,
    request::{Auth, Method, Request, RequestBody},
};

/// Generate a curl command string from a Request
pub fn generate_curl_command(request: &Request) -> String {
    // Build URL with query params and path params
    let url = build_url(&request.url, &request.query_params, &request.path_params);

    // Build first line: curl [method] url
    let has_body = !matches!(request.body, RequestBody::None);
    let first_line = if request.method != Method::GET || (request.method == Method::GET && has_body)
    {
        format!("curl -X {} {}", request.method, shell_quote(&url))
    } else {
        format!("curl {}", shell_quote(&url))
    };

    let mut lines = vec![first_line];

    // Add authentication
    match &request.auth {
        Auth::Basic { username, password } => {
            lines.push(format!(
                "  -u {}",
                shell_quote(&format!("{}:{}", username, password))
            ));
        }
        Auth::Bearer { token } => {
            lines.push(format!(
                "  -H {}",
                shell_quote(&format!("Authorization: Bearer {}", token))
            ));
        }
        Auth::None => {}
    }

    // Add headers (skip disabled ones)
    for header in request.headers.iter() {
        if !header.disabled {
            lines.push(format!(
                "  -H {}",
                shell_quote(&format!("{}: {}", header.name, header.value))
            ));
        }
    }

    // Add body
    add_body_to_lines(&mut lines, &request.body);

    // Join with backslash line continuation
    lines.join(" \\\n")
}

/// Build URL with query params and path params
fn build_url(base_url: &str, query_params: &KeyValList, path_params: &KeyValList) -> String {
    let mut url = base_url.to_string();

    // Substitute path parameters
    for param in path_params.iter() {
        if !param.disabled && !param.name.is_empty() {
            // Replace :param or {param} with the value
            let colon_pattern = format!(":{}", param.name);
            let brace_pattern = format!("{{{}}}", param.name);
            url = url.replace(&colon_pattern, &param.value);
            url = url.replace(&brace_pattern, &param.value);
        }
    }

    // Add query parameters
    let enabled_query_params: Vec<_> = query_params
        .iter()
        .filter(|p| !p.disabled && !p.name.is_empty())
        .collect();

    if !enabled_query_params.is_empty() {
        let separator = if url.contains('?') { '&' } else { '?' };

        let query_string = enabled_query_params
            .iter()
            .map(|p| {
                format!(
                    "{}={}",
                    urlencoding::encode(&p.name),
                    urlencoding::encode(&p.value)
                )
            })
            .collect::<Vec<_>>()
            .join("&");

        url = format!("{}{}{}", url, separator, query_string);
    }

    url
}

/// Add body to curl command lines
fn add_body_to_lines(lines: &mut Vec<String>, body: &RequestBody) {
    match body {
        RequestBody::Json(data) => {
            lines.push(format!(
                "  -H {}",
                shell_quote("Content-Type: application/json")
            ));
            lines.push(format!("  -d {}", shell_quote(data)));
        }
        RequestBody::XML(data) => {
            lines.push(format!(
                "  -H {}",
                shell_quote("Content-Type: application/xml")
            ));
            lines.push(format!("  -d {}", shell_quote(data)));
        }
        RequestBody::Text(data) => {
            lines.push(format!("  -d {}", shell_quote(data)));
        }
        RequestBody::Form(form_data) => {
            for field in form_data.iter() {
                if !field.disabled && !field.name.is_empty() {
                    lines.push(format!(
                        "  -F {}",
                        shell_quote(&format!("{}={}", field.name, field.value))
                    ));
                }
            }
        }
        RequestBody::Multipart { params, files } => {
            // Add form fields
            for field in params.iter() {
                if !field.disabled && !field.name.is_empty() {
                    lines.push(format!(
                        "  -F {}",
                        shell_quote(&format!("{}={}", field.name, field.value))
                    ));
                }
            }
            // Add file uploads
            for file in files.iter() {
                if !file.disabled && !file.name.is_empty() {
                    if let Some(path) = &file.path {
                        let path_str = path.to_string_lossy();
                        lines.push(format!(
                            "  -F {}",
                            shell_quote(&format!("{}=@{}", file.name, path_str))
                        ));
                    }
                }
            }
        }
        RequestBody::File(path) => {
            if let Some(path) = path {
                let path_str = path.to_string_lossy();
                lines.push(format!(
                    "  --data-binary {}",
                    shell_quote(&format!("@{}", path_str))
                ));
            }
        }
        RequestBody::None => {}
    }
}

/// Quote a string for shell use
/// Uses single quotes for safety, escaping any single quotes in the string
fn shell_quote(s: &str) -> String {
    // If the string is simple (alphanumeric, /, ., -, _, :, =), no quoting needed
    if is_safe_unquoted(s) {
        return s.to_string();
    }

    // If string contains single quotes, we need to escape them
    if s.contains('\'') {
        // For single quotes in bash, we need to end the quoted string,
        // add an escaped single quote, and start a new quoted string
        // Example: 'it'\''s' for "it's"
        format!("'{}'", s.replace('\'', r"'\''"))
    } else {
        // Simple case: wrap in single quotes
        format!("'{}'", s)
    }
}

/// Check if a string is safe to use without quotes
fn is_safe_unquoted(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Only allow these characters unquoted
    s.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || c == '/'
            || c == '.'
            || c == '-'
            || c == '_'
            || c == ':'
            || c == '='
            || c == '@'
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{KeyFile, KeyFileList, KeyValList, KeyValue};
    use std::path::PathBuf;

    #[test]
    fn test_simple_get() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/users".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Simple GET: curl and URL on same line
        assert_eq!(cmd, "curl https://api.example.com/users");
    }

    #[test]
    fn test_post_with_json() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/users".to_string(),
            body: RequestBody::Json(r#"{"name":"John"}"#.to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X POST"));
        assert!(cmd.contains("-H 'Content-Type: application/json'"));
        assert!(cmd.contains(r#"-d '{"name":"John"}'"#));
        assert!(cmd.contains(" \\\n")); // Check for line continuation
    }

    #[test]
    fn test_put_with_xml() {
        let req = Request {
            method: Method::PUT,
            url: "https://api.example.com/data".to_string(),
            body: RequestBody::XML("<data><value>test</value></data>".to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X PUT"));
        assert!(cmd.contains("-H 'Content-Type: application/xml'"));
        assert!(cmd.contains("-d '<data><value>test</value></data>'"));
    }

    #[test]
    fn test_basic_auth() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/protected".to_string(),
            auth: Auth::Basic {
                username: "user".to_string(),
                password: "pass".to_string(),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Simple user:pass doesn't need quotes
        assert!(cmd.contains("-u user:pass"));
    }

    #[test]
    fn test_bearer_token() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/data".to_string(),
            auth: Auth::Bearer {
                token: "mytoken123".to_string(),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-H 'Authorization: Bearer mytoken123'"));
    }

    #[test]
    fn test_custom_headers() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com".to_string(),
            headers: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "X-Custom".to_string(),
                    value: "value".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "Accept".to_string(),
                    value: "application/json".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Each header should be on its own line
        assert!(cmd.contains("-H 'X-Custom: value'"));
        assert!(cmd.contains("-H 'Accept: application/json'"));
        assert!(cmd.contains(" \\\n"));
    }

    #[test]
    fn test_disabled_headers_skipped() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com".to_string(),
            headers: KeyValList::from(vec![
                KeyValue {
                    disabled: true,
                    name: "X-Disabled".to_string(),
                    value: "value".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "X-Enabled".to_string(),
                    value: "value".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(!cmd.contains("X-Disabled"));
        assert!(cmd.contains("X-Enabled"));
    }

    #[test]
    fn test_query_params() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/search".to_string(),
            query_params: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "q".to_string(),
                    value: "rust programming".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "limit".to_string(),
                    value: "10".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // URL encoding uses %20 for space
        assert!(cmd.contains("q=rust%20programming"));
        assert!(cmd.contains("limit=10"));
    }

    #[test]
    fn test_query_params_with_existing_query_string() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/search?existing=value".to_string(),
            query_params: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "new".to_string(),
                value: "param".to_string(),
            }]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("?existing=value&new=param"));
    }

    #[test]
    fn test_path_params_colon() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/users/:id/posts".to_string(),
            path_params: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "id".to_string(),
                value: "123".to_string(),
            }]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("users/123/posts"));
    }

    #[test]
    fn test_path_params_braces() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/users/{id}/posts".to_string(),
            path_params: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "id".to_string(),
                value: "456".to_string(),
            }]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("users/456/posts"));
    }

    #[test]
    fn test_form_data() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/form".to_string(),
            body: RequestBody::Form(KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "name".to_string(),
                    value: "John".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "email".to_string(),
                    value: "john@example.com".to_string(),
                },
            ])),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-F name=John"));
        assert!(cmd.contains("-F email=john@example.com"));
    }

    #[test]
    fn test_multipart_with_files() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/upload".to_string(),
            body: RequestBody::Multipart {
                params: KeyValList::from(vec![KeyValue {
                    disabled: false,
                    name: "name".to_string(),
                    value: "John".to_string(),
                }]),
                files: KeyFileList::from(vec![KeyFile {
                    disabled: false,
                    name: "avatar".to_string(),
                    path: Some(PathBuf::from("/path/to/image.jpg")),
                }]),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-F name=John"));
        assert!(cmd.contains("-F avatar=@/path/to/image.jpg"));
    }

    #[test]
    fn test_file_body() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/upload".to_string(),
            body: RequestBody::File(Some(PathBuf::from("/data/file.bin"))),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("--data-binary @/data/file.bin"));
    }

    #[test]
    fn test_shell_quote_simple() {
        assert_eq!(shell_quote("simple"), "simple");
        assert_eq!(shell_quote("with-dash"), "with-dash");
        assert_eq!(shell_quote("with_underscore"), "with_underscore");
        assert_eq!(shell_quote("path/to/file"), "path/to/file");
    }

    #[test]
    fn test_shell_quote_with_spaces() {
        assert_eq!(shell_quote("hello world"), "'hello world'");
    }

    #[test]
    fn test_shell_quote_with_single_quote() {
        assert_eq!(shell_quote("it's"), r"'it'\''s'");
    }

    #[test]
    fn test_shell_quote_with_special_chars() {
        assert_eq!(shell_quote("a&b"), "'a&b'");
        assert_eq!(shell_quote("test;command"), "'test;command'");
        assert_eq!(shell_quote("$(injection)"), "'$(injection)'");
    }

    #[test]
    fn test_delete_method() {
        let req = Request {
            method: Method::DELETE,
            url: "https://api.example.com/users/123".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X DELETE"));
    }

    #[test]
    fn test_patch_method() {
        let req = Request {
            method: Method::PATCH,
            url: "https://api.example.com/users/123".to_string(),
            body: RequestBody::Json(r#"{"status":"updated"}"#.to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X PATCH"));
    }

    #[test]
    fn test_options_method() {
        let req = Request {
            method: Method::OPTIONS,
            url: "https://api.example.com".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X OPTIONS"));
    }

    #[test]
    fn test_head_method() {
        let req = Request {
            method: Method::HEAD,
            url: "https://api.example.com".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X HEAD"));
    }

    #[test]
    fn test_text_body() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Text("plain text data".to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-d 'plain text data'"));
    }

    #[test]
    fn test_complex_request() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/users".to_string(),
            headers: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "X-API-Key".to_string(),
                    value: "secret123".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "Accept".to_string(),
                    value: "application/json".to_string(),
                },
            ]),
            body: RequestBody::Json(r#"{"name":"Jane","email":"jane@example.com"}"#.to_string()),
            query_params: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "notify".to_string(),
                value: "true".to_string(),
            }]),
            auth: Auth::Bearer {
                token: "token123".to_string(),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-X POST"));
        assert!(cmd.contains("-H 'X-API-Key: secret123'"));
        assert!(cmd.contains("-H 'Accept: application/json'"));
        assert!(cmd.contains("-H 'Authorization: Bearer token123'"));
        assert!(cmd.contains("-H 'Content-Type: application/json'"));
        assert!(cmd.contains(r#"-d '{"name":"Jane","email":"jane@example.com"}'"#));
        assert!(cmd.contains("?notify=true"));
    }

    #[test]
    fn test_url_encoding_special_chars() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/search".to_string(),
            query_params: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "q".to_string(),
                value: "hello world!@#$%".to_string(),
            }]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Should be URL encoded (space becomes %20, special chars are encoded)
        assert!(cmd.contains("q=hello%20world"));
        assert!(cmd.contains("%21")); // ! encoded
    }

    #[test]
    fn test_disabled_query_params_skipped() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com".to_string(),
            query_params: KeyValList::from(vec![
                KeyValue {
                    disabled: true,
                    name: "skip".to_string(),
                    value: "me".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "include".to_string(),
                    value: "me".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(!cmd.contains("skip"));
        assert!(cmd.contains("include=me"));
    }

    #[test]
    fn test_disabled_path_params_skipped() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/:id/:type".to_string(),
            path_params: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "id".to_string(),
                    value: "123".to_string(),
                },
                KeyValue {
                    disabled: true,
                    name: "type".to_string(),
                    value: "user".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("123"));
        assert!(cmd.contains(":type")); // Not replaced
    }

    #[test]
    fn test_disabled_form_fields_skipped() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Form(KeyValList::from(vec![
                KeyValue {
                    disabled: true,
                    name: "skip".to_string(),
                    value: "me".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "include".to_string(),
                    value: "me".to_string(),
                },
            ])),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(!cmd.contains("skip"));
        assert!(cmd.contains("-F include=me"));
    }

    #[test]
    fn test_disabled_multipart_files_skipped() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Multipart {
                params: KeyValList::from(vec![]),
                files: KeyFileList::from(vec![
                    KeyFile {
                        disabled: true,
                        name: "skip".to_string(),
                        path: Some(PathBuf::from("/skip.jpg")),
                    },
                    KeyFile {
                        disabled: false,
                        name: "include".to_string(),
                        path: Some(PathBuf::from("/include.jpg")),
                    },
                ]),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(!cmd.contains("skip"));
        assert!(cmd.contains("include=@/include.jpg"));
    }

    #[test]
    fn test_empty_string_values() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com".to_string(),
            headers: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "X-Empty".to_string(),
                value: "".to_string(),
            }]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-H 'X-Empty: '"));
    }

    #[test]
    fn test_basic_auth_with_special_chars() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com".to_string(),
            auth: Auth::Basic {
                username: "user@domain.com".to_string(),
                password: "p@ss:word!".to_string(),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-u 'user@domain.com:p@ss:word!'"));
    }

    #[test]
    fn test_json_with_quotes() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Json(r#"{"message":"He said \"Hello\""}"#.to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // The shell quoting should handle the quotes properly
        assert!(cmd.contains("-d"));
    }

    #[test]
    fn test_url_with_port() {
        let req = Request {
            method: Method::GET,
            url: "http://localhost:8080/api".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("localhost:8080"));
    }

    #[test]
    fn test_url_with_auth_credentials() {
        let req = Request {
            method: Method::GET,
            url: "https://user:pass@api.example.com".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("user:pass@api.example.com"));
    }

    #[test]
    fn test_multiple_path_params() {
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/users/{userId}/posts/{postId}".to_string(),
            path_params: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "userId".to_string(),
                    value: "123".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "postId".to_string(),
                    value: "456".to_string(),
                },
            ]),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("users/123/posts/456"));
    }

    #[test]
    fn test_get_with_body() {
        // Some APIs accept GET with body (elasticsearch for example)
        let req = Request {
            method: Method::GET,
            url: "https://api.example.com/search".to_string(),
            body: RequestBody::Json(r#"{"query":"test"}"#.to_string()),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Should include -X GET when there's a body
        assert!(cmd.contains("-X GET"));
        assert!(cmd.contains("-d"));
    }

    #[test]
    fn test_file_with_no_path() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::File(None),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Should not add --data-binary if path is None
        assert!(!cmd.contains("--data-binary"));
    }

    #[test]
    fn test_multipart_file_with_no_path() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Multipart {
                params: KeyValList::from(vec![]),
                files: KeyFileList::from(vec![KeyFile {
                    disabled: false,
                    name: "file".to_string(),
                    path: None,
                }]),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        // Should not add file upload if path is None
        assert!(!cmd.contains("file=@"));
    }

    #[test]
    fn test_form_field_with_equals_in_value() {
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com".to_string(),
            body: RequestBody::Form(KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "data".to_string(),
                value: "key=value".to_string(),
            }])),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("-F data=key=value"));
    }

    #[test]
    fn test_ipv6_url() {
        let req = Request {
            method: Method::GET,
            url: "http://[2001:db8::1]:8080/api".to_string(),
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);
        assert!(cmd.contains("[2001:db8::1]:8080"));
    }

    #[test]
    fn test_multiline_format() {
        // Test to demonstrate the multi-line format
        let req = Request {
            method: Method::POST,
            url: "https://api.example.com/users".to_string(),
            headers: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "X-API-Key".to_string(),
                    value: "secret123".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "Accept".to_string(),
                    value: "application/json".to_string(),
                },
            ]),
            body: RequestBody::Json(r#"{"name":"Jane","email":"jane@example.com"}"#.to_string()),
            auth: Auth::Bearer {
                token: "token123".to_string(),
            },
            ..Default::default()
        };
        let cmd = generate_curl_command(&req);

        // Print the command to see the format
        println!("\nGenerated curl command:");
        println!("{}", cmd);

        // Verify structure: first line should have curl, method, and url
        let lines: Vec<&str> = cmd.split(" \\\n").collect();
        assert!(lines[0].starts_with("curl -X POST"));
        assert!(lines[0].contains("https://api.example.com/users"));
        assert!(lines.iter().any(|l| l.contains("Authorization: Bearer")));
        assert!(lines.iter().any(|l| l.contains("X-API-Key")));
        assert!(
            lines
                .iter()
                .any(|l| l.contains("Content-Type: application/json"))
        );
        assert!(lines.iter().any(|l| l.contains("-d")));
    }

    // Integration tests demonstrating round-trip functionality
    #[test]
    fn test_roundtrip_simple_post() {
        use crate::curl::parse_curl_command;

        // Create a request
        let original_req = Request {
            method: Method::POST,
            url: "https://api.example.com/users".to_string(),
            headers: KeyValList::from(vec![KeyValue {
                disabled: false,
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }]),
            body: RequestBody::Json(r#"{"name":"Alice","age":30}"#.to_string()),
            ..Default::default()
        };

        // Generate curl command
        let curl_cmd = generate_curl_command(&original_req);

        // Parse it back
        let parsed_req = parse_curl_command(&curl_cmd).expect("Failed to parse generated curl");

        // Verify key properties match
        assert_eq!(parsed_req.method, original_req.method);
        assert_eq!(parsed_req.url, original_req.url);
        assert_eq!(parsed_req.body, original_req.body);

        // Content-Type header should be present (either from original or added by generator)
        assert!(parsed_req.headers.iter().any(|h| h.name == "Content-Type"));
    }

    #[test]
    fn test_roundtrip_complex_request() {
        use crate::curl::parse_curl_command;

        let original_req = Request {
            method: Method::PUT,
            url: "https://api.example.com/items/123".to_string(),
            headers: KeyValList::from(vec![
                KeyValue {
                    disabled: false,
                    name: "Authorization".to_string(),
                    value: "Bearer token123".to_string(),
                },
                KeyValue {
                    disabled: false,
                    name: "X-Custom-Header".to_string(),
                    value: "custom-value".to_string(),
                },
            ]),
            body: RequestBody::Json(r#"{"status":"active"}"#.to_string()),
            auth: Auth::None, // Auth is in header
            ..Default::default()
        };

        let curl_cmd = generate_curl_command(&original_req);
        let parsed_req = parse_curl_command(&curl_cmd).expect("Failed to parse");

        assert_eq!(parsed_req.method, original_req.method);
        assert_eq!(parsed_req.url, original_req.url);
        assert_eq!(parsed_req.body, original_req.body);

        // All headers should be present
        assert!(parsed_req.headers.iter().any(|h| h.name == "Authorization"));
        assert!(
            parsed_req
                .headers
                .iter()
                .any(|h| h.name == "X-Custom-Header")
        );
    }
}
