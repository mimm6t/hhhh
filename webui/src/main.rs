use std::fs;
use std::io::Read;
use std::process::Command;
use tiny_http::{Response, Server};

const SCRIPTS_DIR: &str = "/data/adb/rustfrida/scripts";
const HOOKS_CONFIG: &str = "/data/adb/rustfrida/hooks.json";

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("rustFrida Web UI listening on 0.0.0.0:8080");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().as_str();
        
        let response = match (method, url.as_str()) {
            ("GET", "/") => {
                Response::from_string(include_str!("index.html"))
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap())
            }
            ("GET", "/api/apps") => get_apps(),
            ("GET", "/api/scripts") => get_scripts(),
            ("GET", "/api/hooks") => get_hooks(),
            ("GET", "/api/output") => get_logs(),
            ("GET", path) if path.starts_with("/api/script/") => {
                let name = path.trim_start_matches("/api/script/");
                get_script(name)
            }
            ("POST", "/api/script/save") => save_script(&mut request),
            ("POST", "/api/script/delete") => delete_script(&mut request),
            ("POST", "/api/hook/enable") => enable_hook(&mut request),
            ("POST", "/api/hook/disable") => disable_hook(&mut request),
            ("POST", "/api/inject") => inject_now(&mut request),
            _ => Response::from_string("Not Found").with_status_code(404),
        };
        
        let _ = request.respond(response);
    }
}

fn json_response(json: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(json)
        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn get_apps() -> Response<std::io::Cursor<Vec<u8>>> {
    let output = Command::new("pm").args(&["list", "packages", "-3"]).output();
    let packages: Vec<_> = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|pkg| format!(r#"{{"package":"{}","name":"{}"}}"#, pkg, pkg))
            .collect(),
        Err(_) => vec![],
    };
    json_response(&format!("[{}]", packages.join(",")))
}

fn get_scripts() -> Response<std::io::Cursor<Vec<u8>>> {
    let mut scripts = Vec::new();
    if let Ok(entries) = fs::read_dir(SCRIPTS_DIR) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".js") {
                    scripts.push(format!(r#"{{"name":"{}"}}"#, name));
                }
            }
        }
    }
    json_response(&format!("[{}]", scripts.join(",")))
}

fn get_script(name: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let path = format!("{}/{}", SCRIPTS_DIR, name);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let escaped = content.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
            json_response(&format!(r#"{{"content":"{}"}}"#, escaped))
        }
        Err(_) => Response::from_string(r#"{"error":"Not found"}"#).with_status_code(404),
    }
}

fn get_hooks() -> Response<std::io::Cursor<Vec<u8>>> {
    match fs::read_to_string(HOOKS_CONFIG) {
        Ok(content) => json_response(&content),
        Err(_) => json_response(r#"{"enabled":[],"hooks":{}}"#),
    }
}

fn get_logs() -> Response<std::io::Cursor<Vec<u8>>> {
    let mut logs = Vec::new();
    
    // inject.log - 最重要，放最前面
    if let Ok(content) = fs::read_to_string("/data/adb/modules/rustfrida-kernelsu/logs/inject.log") {
        let lines: Vec<&str> = content.lines().collect();
        let start = if lines.len() > 100 { lines.len() - 100 } else { 0 };
        logs.push(format!("=== inject.log (last 100 lines) ===\n{}", lines[start..].join("\n")));
    }
    
    // rustfrida.log - 最后50行
    if let Ok(content) = fs::read_to_string("/data/adb/modules/rustfrida-kernelsu/logs/rustfrida.log") {
        let lines: Vec<&str> = content.lines().collect();
        let start = if lines.len() > 50 { lines.len() - 50 } else { 0 };
        logs.push(format!("\n=== rustfrida.log (last 50 lines) ===\n{}", lines[start..].join("\n")));
    }
    
    // auto-hook.log
    if let Ok(content) = fs::read_to_string("/data/adb/modules/rustfrida-kernelsu/logs/auto-hook.log") {
        logs.push(format!("\n=== auto-hook.log ===\n{}", content));
    }
    
    // webui.log
    if let Ok(content) = fs::read_to_string("/data/adb/modules/rustfrida-kernelsu/logs/webui.log") {
        logs.push(format!("\n=== webui.log ===\n{}", content));
    }
    
    let output = logs.join("\n").replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    json_response(&format!(r#"{{"output":"{}"}}"#, output))
}

fn read_body(request: &mut tiny_http::Request) -> Result<String, ()> {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).map_err(|_| ())?;
    Ok(content)
}

fn save_script(request: &mut tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = match read_body(request) {
        Ok(b) => b,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    if let (Some(name), Some(content)) = (parsed["name"].as_str(), parsed["content"].as_str()) {
        let path = format!("{}/{}", SCRIPTS_DIR, name);
        if fs::write(&path, content).is_ok() {
            return json_response(r#"{"success":true}"#);
        }
    }
    json_response(r#"{"success":false}"#)
}

fn delete_script(request: &mut tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = match read_body(request) {
        Ok(b) => b,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    if let Some(name) = parsed["name"].as_str() {
        let path = format!("{}/{}", SCRIPTS_DIR, name);
        if fs::remove_file(&path).is_ok() {
            return json_response(r#"{"success":true}"#);
        }
    }
    json_response(r#"{"success":false}"#)
}

fn enable_hook(request: &mut tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = match read_body(request) {
        Ok(b) => b,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    let req: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    if let Some(package) = req["package"].as_str() {
        let mut config: serde_json::Value = fs::read_to_string(HOOKS_CONFIG)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| serde_json::json!({"enabled":[],"hooks":{}}));
        
        if let Some(enabled) = config["enabled"].as_array_mut() {
            if !enabled.iter().any(|v| v.as_str() == Some(package)) {
                enabled.push(serde_json::Value::String(package.to_string()));
            }
        }
        
        if let Some(script) = req["script"].as_str() {
            if let Some(hooks) = config["hooks"].as_object_mut() {
                hooks.insert(package.to_string(), serde_json::Value::String(script.to_string()));
            }
        }
        
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            if fs::write(HOOKS_CONFIG, json).is_ok() {
                return json_response(r#"{"success":true}"#);
            }
        }
    }
    json_response(r#"{"success":false}"#)
}

fn disable_hook(request: &mut tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = match read_body(request) {
        Ok(b) => b,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    let req: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"success":false}"#),
    };
    
    if let Some(package) = req["package"].as_str() {
        let mut config: serde_json::Value = fs::read_to_string(HOOKS_CONFIG)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| serde_json::json!({"enabled":[],"hooks":{}}));
        
        if let Some(enabled) = config["enabled"].as_array_mut() {
            enabled.retain(|v| v.as_str() != Some(package));
        }
        
        if let Some(hooks) = config["hooks"].as_object_mut() {
            hooks.remove(package);
        }
        
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            if fs::write(HOOKS_CONFIG, json).is_ok() {
                return json_response(r#"{"success":true}"#);
            }
        }
    }
    json_response(r#"{"success":false}"#)
}

fn inject_now(request: &mut tiny_http::Request) -> Response<std::io::Cursor<Vec<u8>>> {
    use std::io::Write;
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut log_file = match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/data/adb/modules/rustfrida-kernelsu/logs/inject.log") {
        Ok(f) => f,
        Err(e) => {
            return json_response(&format!(r#"{{"success":false,"error":"Cannot open log: {}"}}"#, e));
        }
    };
    
    let _ = writeln!(log_file, "\n[{}] ===== Injection Request =====", timestamp);
    
    let body = match read_body(request) {
        Ok(b) => {
            let _ = writeln!(log_file, "[{}] Request body: {}", timestamp, b);
            b
        },
        Err(_) => {
            let _ = writeln!(log_file, "[{}] ERROR: Failed to read body", timestamp);
            return json_response(r#"{"success":false,"error":"Failed to read body"}"#);
        }
    };
    
    let req: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => {
            let _ = writeln!(log_file, "[{}] Parsed JSON: {:?}", timestamp, v);
            v
        },
        Err(e) => {
            let _ = writeln!(log_file, "[{}] ERROR: Invalid JSON: {}", timestamp, e);
            return json_response(r#"{"success":false,"error":"Invalid JSON"}"#);
        }
    };
    
    if let (Some(package), Some(script)) = (req["package"].as_str(), req["script"].as_str()) {
        let _ = writeln!(log_file, "[{}] Package: {}", timestamp, package);
        let _ = writeln!(log_file, "[{}] Script: {}", timestamp, script);
        
        let script_path = format!("{}/{}", SCRIPTS_DIR, script);
        let _ = writeln!(log_file, "[{}] Script path: {}", timestamp, script_path);
        
        // 检查脚本是否存在
        if !std::path::Path::new(&script_path).exists() {
            let _ = writeln!(log_file, "[{}] ERROR: Script not found at {}", timestamp, script_path);
            return json_response(r#"{"success":false,"error":"Script not found"}"#);
        }
        
        let _ = writeln!(log_file, "[{}] Script exists, starting injection...", timestamp);
        
        let rustfrida_bin = "/data/adb/modules/rustfrida-kernelsu/bin/rustfrida";
        let args = vec!["--spawn", package, "-l", &script_path];
        
        let _ = writeln!(log_file, "[{}] Command: {} {:?}", timestamp, rustfrida_bin, args);
        
        // 使用 spawn 模式注入
        let output = Command::new(rustfrida_bin)
            .args(&args)
            .output();
        
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let status = out.status.code().unwrap_or(-1);
                
                let _ = writeln!(log_file, "[{}] Exit code: {}", timestamp, status);
                let _ = writeln!(log_file, "[{}] STDOUT:\n{}", timestamp, stdout);
                let _ = writeln!(log_file, "[{}] STDERR:\n{}", timestamp, stderr);
                
                if out.status.success() {
                    let _ = writeln!(log_file, "[{}] SUCCESS: Injection completed", timestamp);
                    json_response(r#"{"success":true,"message":"Injection started"}"#)
                } else {
                    let _ = writeln!(log_file, "[{}] FAILED: Injection failed", timestamp);
                    let error = format!(r#"{{"success":false,"error":"Exit code {}: {}"}}"#, 
                        status, stderr.replace('"', "\\\"").replace('\n', " "));
                    json_response(&error)
                }
            }
            Err(e) => {
                let _ = writeln!(log_file, "[{}] ERROR: Failed to execute: {}", timestamp, e);
                let error = format!(r#"{{"success":false,"error":"{}"}}"#, e.to_string().replace('"', "\\\""));
                json_response(&error)
            }
        }
    } else {
        let _ = writeln!(log_file, "[{}] ERROR: Missing package or script in request", timestamp);
        json_response(r#"{"success":false,"error":"Missing package or script"}"#)
    }
}
