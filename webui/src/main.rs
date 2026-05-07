use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use tokio::fs;

const RUSTFRIDA_BIN: &str = "/data/adb/modules/rustfrida-kernelsu/bin/rustfrida";
const SCRIPTS_DIR: &str = "/data/adb/rustfrida/scripts";
const HOOKS_CONFIG: &str = "/data/adb/rustfrida/hooks.json";

struct AppState {
    output: Mutex<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct HooksConfig {
    enabled: Vec<String>,
    hooks: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
struct HookRequest {
    package: String,
    script: Option<String>,
}

#[derive(Deserialize)]
struct ScriptRequest {
    name: String,
    content: Option<String>,
}

async fn index() -> Result<HttpResponse> {
    let html = include_str!("index.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn get_apps() -> Result<HttpResponse> {
    let output = Command::new("pm")
        .args(&["list", "packages", "-3"])
        .output()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to list apps"))?;
    
    let packages: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("package:"))
        .map(|pkg| serde_json::json!({"package": pkg, "name": pkg}))
        .collect();
    
    Ok(HttpResponse::Ok().json(packages))
}

async fn get_scripts() -> Result<HttpResponse> {
    let mut scripts = Vec::new();
    if let Ok(mut entries) = fs::read_dir(SCRIPTS_DIR).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".js") {
                    scripts.push(serde_json::json!({"name": name}));
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(scripts))
}

async fn get_script(name: web::Path<String>) -> Result<HttpResponse> {
    let path = format!("{}/{}", SCRIPTS_DIR, name.as_str());
    match fs::read_to_string(&path).await {
        Ok(content) => Ok(HttpResponse::Ok().json(serde_json::json!({"content": content}))),
        Err(_) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "Script not found"}))),
    }
}

async fn save_script(req: web::Json<ScriptRequest>) -> Result<HttpResponse> {
    let path = format!("{}/{}", SCRIPTS_DIR, req.name);
    if let Some(content) = &req.content {
        fs::write(&path, content).await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save script"))?;
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}

async fn delete_script(req: web::Json<ScriptRequest>) -> Result<HttpResponse> {
    let path = format!("{}/{}", SCRIPTS_DIR, req.name);
    fs::remove_file(&path).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to delete script"))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}

async fn get_hooks() -> Result<HttpResponse> {
    match fs::read_to_string(HOOKS_CONFIG).await {
        Ok(content) => {
            let config: HooksConfig = serde_json::from_str(&content).unwrap_or_else(|_| HooksConfig {
                enabled: Vec::new(),
                hooks: std::collections::HashMap::new(),
            });
            Ok(HttpResponse::Ok().json(config))
        }
        Err(_) => Ok(HttpResponse::Ok().json(HooksConfig {
            enabled: Vec::new(),
            hooks: std::collections::HashMap::new(),
        })),
    }
}

async fn enable_hook(req: web::Json<HookRequest>) -> Result<HttpResponse> {
    let mut config: HooksConfig = match fs::read_to_string(HOOKS_CONFIG).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| HooksConfig {
            enabled: Vec::new(),
            hooks: std::collections::HashMap::new(),
        }),
        Err(_) => HooksConfig {
            enabled: Vec::new(),
            hooks: std::collections::HashMap::new(),
        },
    };
    
    if !config.enabled.contains(&req.package) {
        config.enabled.push(req.package.clone());
    }
    if let Some(script) = &req.script {
        config.hooks.insert(req.package.clone(), script.clone());
    }
    
    let json = serde_json::to_string_pretty(&config)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to serialize config"))?;
    fs::write(HOOKS_CONFIG, json).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save config"))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}

async fn disable_hook(req: web::Json<HookRequest>) -> Result<HttpResponse> {
    let mut config: HooksConfig = match fs::read_to_string(HOOKS_CONFIG).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| HooksConfig {
            enabled: Vec::new(),
            hooks: std::collections::HashMap::new(),
        }),
        Err(_) => HooksConfig {
            enabled: Vec::new(),
            hooks: std::collections::HashMap::new(),
        },
    };
    
    config.enabled.retain(|p| p != &req.package);
    config.hooks.remove(&req.package);
    
    let json = serde_json::to_string_pretty(&config)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to serialize config"))?;
    fs::write(HOOKS_CONFIG, json).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save config"))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}

async fn get_output(data: web::Data<AppState>) -> Result<HttpResponse> {
    let output = data.output.lock().unwrap();
    Ok(HttpResponse::Ok().json(serde_json::json!({"output": output.join("\n")})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        output: Mutex::new(Vec::new()),
    });
    
    println!("Starting rustFrida Web UI on 0.0.0.0:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/api/apps", web::get().to(get_apps))
            .route("/api/scripts", web::get().to(get_scripts))
            .route("/api/script/{name}", web::get().to(get_script))
            .route("/api/script/save", web::post().to(save_script))
            .route("/api/script/delete", web::post().to(delete_script))
            .route("/api/hooks", web::get().to(get_hooks))
            .route("/api/hook/enable", web::post().to(enable_hook))
            .route("/api/hook/disable", web::post().to(disable_hook))
            .route("/api/output", web::get().to(get_output))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
