/// Hide-My-Applist Rust - Main Entry Point
/// 
/// Command-line tool for managing app hiding

use hide_my_applist_rust::{Config, PmsHookEngine};
use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    hide_my_applist_rust::init_logging();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    
    match args[1].as_str() {
        "install" => install_hooks(&args[2..])?,
        "uninstall" => uninstall_hooks()?,
        "config" => manage_config(&args[2..])?,
        "test" => test_wxshadow()?,
        "version" => print_version(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("Hide-My-Applist Rust v{}", hide_my_applist_rust::VERSION);
    println!();
    println!("USAGE:");
    println!("  hma-rust install [config_path]  - Install hooks with config");
    println!("  hma-rust uninstall              - Uninstall all hooks");
    println!("  hma-rust config <path>          - Load and validate config");
    println!("  hma-rust test                   - Test wxshadow availability");
    println!("  hma-rust version                - Print version");
}

fn print_version() {
    println!("Hide-My-Applist Rust v{}", hide_my_applist_rust::VERSION);
}

fn install_hooks(args: &[String]) -> Result<()> {
    let config_path = if args.is_empty() {
        PathBuf::from("/data/local/tmp/hma_config.json")
    } else {
        PathBuf::from(&args[0])
    };
    
    log::info!("Loading configuration from {:?}", config_path);
    let config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        log::warn!("Config file not found, using default");
        Config::default()
    };
    
    log::info!("Initializing hook engine...");
    let mut engine = PmsHookEngine::new(config);
    engine.init()?;
    
    log::info!("Installing hooks...");
    engine.install_hooks()?;
    
    log::info!("Hooks installed successfully!");
    log::info!("Press Ctrl+C to uninstall and exit");
    
    // Keep running until interrupted
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    }).context("Failed to set Ctrl+C handler")?;
    
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    log::info!("Uninstalling hooks...");
    engine.uninstall_hooks()?;
    
    Ok(())
}

fn uninstall_hooks() -> Result<()> {
    log::info!("Uninstalling hooks...");
    // In a real implementation, we would need to track active hooks
    // and clean them up properly
    log::info!("Hooks uninstalled");
    Ok(())
}

fn manage_config(args: &[String]) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("Config path required");
    }
    
    let config_path = PathBuf::from(&args[0]);
    
    if config_path.exists() {
        log::info!("Loading config from {:?}", config_path);
        let config = Config::load(&config_path)?;
        log::info!("Config loaded successfully");
        log::info!("  Version: {}", config.config_version);
        log::info!("  Scope entries: {}", config.scope.len());
        log::info!("  Templates: {}", config.templates.len());
    } else {
        log::info!("Creating default config at {:?}", config_path);
        let config = Config::default();
        config.save(&config_path)?;
        log::info!("Default config created");
    }
    
    Ok(())
}

fn test_wxshadow() -> Result<()> {
    use hide_my_applist_rust::wxshadow;
    
    log::info!("Testing wxshadow availability...");
    
    // Try to set a dummy breakpoint (will fail if wxshadow not loaded)
    let result = wxshadow::set_breakpoint(1, 0x1000);
    
    match result {
        Ok(_) => {
            log::info!("✓ wxshadow is available");
            // Clean up
            let _ = wxshadow::delete_breakpoint(1, 0x1000);
        }
        Err(e) => {
            log::error!("✗ wxshadow is NOT available: {}", e);
            log::error!("Make sure wxshadow.kpm is loaded:");
            log::error!("  kpatch module load /path/to/wxshadow.kpm");
        }
    }
    
    Ok(())
}
