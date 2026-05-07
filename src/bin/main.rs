/// Hide-My-Applist Rust CLI
use anyhow::Result;
use hide_my_applist_rust::{Config, PmsHookEngine, init_logging};
use std::env;

const VERSION: &str = "0.2.0";

fn main() -> Result<()> {
    init_logging();
    
    let args: Vec<String> = env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("version") | Some("-v") | Some("--version") => {
            println!("Hide-My-Applist Rust v{}", VERSION);
            Ok(())
        }
        Some("help") | Some("-h") | Some("--help") => {
            print_help();
            Ok(())
        }
        Some("test") => {
            test_hook_engine()
        }
        Some("config") => {
            show_config()
        }
        _ => {
            println!("Hide-My-Applist Rust v{}", VERSION);
            println!("A Frida-based app hiding tool for Android");
            println!();
            println!("Usage: hma-rust [COMMAND]");
            println!();
            println!("Commands:");
            println!("  version    Show version information");
            println!("  help       Show this help message");
            println!("  test       Test hook engine");
            println!("  config     Show default configuration");
            Ok(())
        }
    }
}

fn print_help() {
    println!("Hide-My-Applist Rust v{}", VERSION);
    println!("A Frida-based app hiding tool for Android");
    println!();
    println!("USAGE:");
    println!("    hma-rust [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    version    Show version information");
    println!("    help       Show this help message");
    println!("    test       Test hook engine initialization");
    println!("    config     Show default configuration");
    println!();
    println!("EXAMPLES:");
    println!("    hma-rust version");
    println!("    hma-rust test");
    println!("    hma-rust config");
}

fn test_hook_engine() -> Result<()> {
    println!("Testing Hook Engine...");
    
    let config = Config::new();
    println!("✓ Config created");
    
    let mut engine = PmsHookEngine::new(config);
    println!("✓ Hook engine created");
    
    match engine.init() {
        Ok(_) => println!("✓ Hook engine initialized"),
        Err(e) => println!("✗ Hook engine init failed: {}", e),
    }
    
    #[cfg(feature = "frida-gum")]
    println!("✓ Frida-Gum support enabled");
    
    #[cfg(not(feature = "frida-gum"))]
    println!("⚠ Frida-Gum support disabled");
    
    println!("Hook engine test completed");
    Ok(())
}

fn show_config() -> Result<()> {
    println!("Default Configuration:");
    
    let mut config = Config::new();
    config.load_default_templates();
    
    match config.to_json() {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error serializing config: {}", e),
    }
    
    Ok(())
}