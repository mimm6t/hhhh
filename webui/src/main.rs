use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tiny_http::{Response, Server};

const SCRIPTS_DIR: &str = "/data/adb/rustfrida/scripts";
const HOOKS_CONFIG: &str = "/data/adb/rustfrida/hooks.json";

#[derive(Serialize, Deserialize)]
struct HooksConfig {
    enabled: Vec<String>,
    hooks: HashMap<String, String>,
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("rustFrida Web UI listening on 0.0.0.0:8080");

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        
        let response = if url == "/" {
            Response::from_string(include_str!("index.html"))
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap())
        } else if url == "/api/apps" {
            handle_apps()
        } else if url == "/api/scripts" {
            handle_scripts()
        } else if url.starts_with("/api/script/") && request.method().as_str() == "GET" {
            let name = url.trim_start_matches("/api/script/");
            handle_get_script(name)
        } else if url == "/api/script/save" && request.method().as_str() == "POST" {
            handle_save_script(&request)
        } else if url == "/api/script/delete" && request.method().as_str() == "POST" {
            handle_delete_script(&request)
        } else if url == "/api/hooks" {
            handle_get_hooks()
        } else if url == "/api/hook/enable" && request.method().as_str() == "POST" {
            handle_enable_hook(&request)
        } else if url == "/api/hook/disable" && request.method().as_str() == "POST" {
            handle_disable_hook(&request)
        } else if url == "/api/output" {
            Response::from_string(r#"{"output":""}"#)
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
        } else {
            Response::from_string("Not Found").with_status_code(404)
        };
        
        let _ = request.respond(response);
    }
}

fn handle_apps() -> Response<std::io::Cursor<Vec<u8>>> {
    let output = Command::new("pm")
        .args(&["list", "packages", "-3"])
        .output();
    
    let packages: Vec<_> = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|pkg| serde_json::json!({"package": pkg, "name": pkg}))
            .collect(),
        Err(_) => vec![],
    };
    
    Response::from_string(serde_json::to_string(&packages).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_scripts() -> Response<std::io::Cursor<Vec<u8>>> {
    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(SCRIPTS_DIR) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".js") {
                    scripts.push(serde_json::json!({"name": name}));
                }
            }
        }
    }
    Response::from_string(serde_json::to_string(&scripts).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_get_script(name: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let path = format!("{}/{}", SCRIPTS_DIR, name);
    match fs::read_to_string(&path) {
        Ok(content) => Response::from_string(serde_json::json!({"content": content}).to_string())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()),
        Err(_) => Response::from_string(r#"{"error":"Not found"}"#).with_status_code(404)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()),
    }
}

fn handle_save_script(request: &tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return Response::from_string(r#"{"success":false}"#)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
    }
    
    if let Ok(req) = serde_json::from_str::<serde_json::Value>(&content) {
        if let (Some(name), Some(script_content)) = (req["name"].as_str(), req["content"].as_str()) {
            let path = format!("{}/{}", SCRIPTS_DIR, name);
            if fs::write(&path, script_content).is_ok() {
                return Response::from_string(r#"{"success":true}"#)
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            }
        }
    }
    Response::from_string(r#"{"success":false}"#)
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_delete_script(request: &tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return Response::from_string(r#"{"success":false}"#)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
    }
    
    if let Ok(req) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(name) = req["name"].as_str() {
            let path = format!("{}/{}", SCRIPTS_DIR, name);
            if fs::remove_file(&path).is_ok() {
                return Response::from_string(r#"{"success":true}"#)
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            }
        }
    }
    Response::from_string(r#"{"success":false}"#)
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_get_hooks() -> Response<std::io::Cursor<Vec<u8>>> {
    let config: HooksConfig = fs::read_to_string(HOOKS_CONFIG)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| HooksConfig {
            enabled: Vec::new(),
            hooks: HashMap::new(),
        });
    
    Response::from_string(serde_json::to_string(&config).unwrap())
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_enable_hook(request: &tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return Response::from_string(r#"{"success":false}"#)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
    }
    
    if let Ok(req) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(package) = req["package"].as_str() {
            let mut config: HooksConfig = fs::read_to_string(HOOKS_CONFIG)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_else(|| HooksConfig {
                    enabled: Vec::new(),
                    hooks: HashMap::new(),
                });
            
            if !config.enabled.contains(&package.to_string()) {
                config.enabled.push(package.to_string());
            }
            if let Some(script) = req["script"].as_str() {
                config.hooks.insert(package.to_string(), script.to_string());
            }
            
            if let Ok(json) = serde_json::to_string_pretty(&config) {
                if fs::write(HOOKS_CONFIG, json).is_ok() {
                    return Response::from_string(r#"{"success":true}"#)
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                }
            }
        }
    }
    Response::from_string(r#"{"success":false}"#)
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn handle_disable_hook(request: &tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut content = String::new();
    if let Err(_) = request.as_reader().read_to_string(&mut content) {
        return Response::from_string(r#"{"success":false}"#)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
    }
    
    if let Ok(req) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(package) = req["package"].as_str() {
            let mut config: HooksConfig = fs::read_to_string(HOOKS_CONFIG)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_else(|| HooksConfig {
                    enabled: Vec::new(),
                    hooks: HashMap::new(),
                });
            
            config.enabled.retain(|p| p != package);
            config.hooks.remove(package);
            
            if let Ok(json) = serde_json::to_string_pretty(&config) {
                if fs::write(HOOKS_CONFIG, json).is_ok() {
                    return Response::from_string(r#"{"success":true}"#)
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                }
            }
        }
    }
    Response::from_string(r#"{"success":false}"#)
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}
