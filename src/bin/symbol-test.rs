/// Symbol resolution test tool
use hide_my_applist_rust::{elf, symbol, android, process};
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    
    match args[1].as_str() {
        "parse" => parse_elf(&args[2..])?,
        "resolve" => resolve_symbol(&args[2..])?,
        "version" => check_version()?,
        "maps" => show_maps(&args[2..])?,
        _ => print_usage(),
    }
    
    Ok(())
}

fn print_usage() {
    println!("Symbol Resolution Test Tool");
    println!();
    println!("USAGE:");
    println!("  symbol-test parse <elf_file>           - Parse ELF symbols");
    println!("  symbol-test resolve <pid> <lib> <sym>  - Resolve symbol in process");
    println!("  symbol-test version                    - Check Android version");
    println!("  symbol-test maps <pid>                 - Show process memory maps");
}

fn parse_elf(args: &[String]) -> Result<()> {
    if args.is_empty() {
        println!("Error: ELF file path required");
        return Ok(());
    }
    
    let path = &args[0];
    println!("Parsing ELF: {}", path);
    
    let symbols = elf::parse_symbols(path)?;
    println!("Found {} symbols", symbols.len());
    
    // Show first 20 symbols
    for (i, sym) in symbols.iter().take(20).enumerate() {
        println!("{:4}. 0x{:016x} {:8} {}", i + 1, sym.addr, sym.size, sym.name);
    }
    
    if symbols.len() > 20 {
        println!("... and {} more", symbols.len() - 20);
    }
    
    Ok(())
}

fn resolve_symbol(args: &[String]) -> Result<()> {
    if args.len() < 3 {
        println!("Error: pid, library, and symbol name required");
        return Ok(());
    }
    
    let pid: i32 = args[0].parse()?;
    let lib = &args[1];
    let sym = &args[2];
    
    println!("Resolving symbol in process {}", pid);
    println!("Library: {}", lib);
    println!("Symbol: {}", sym);
    
    let mut resolver = symbol::SymbolResolver::new();
    
    match resolver.resolve(pid, lib, sym)? {
        Some(addr) => {
            println!("✓ Found at: 0x{:x}", addr);
        }
        None => {
            println!("✗ Symbol not found");
        }
    }
    
    Ok(())
}

fn check_version() -> Result<()> {
    let version = android::AndroidVersion::detect()?;
    println!("Android Version: {:?}", version);
    println!("SDK Int: {}", version.sdk_int());
    
    let targets = android::PmsHookTargets::for_version(version);
    println!("\nHook Targets:");
    
    if let Some(ref t) = targets.should_filter_application {
        println!("  - shouldFilterApplication: {}", t);
    }
    if let Some(ref t) = targets.get_packages_for_uid {
        println!("  - getPackagesForUid: {}", t);
    }
    if let Some(ref t) = targets.get_installed_packages {
        println!("  - getInstalledPackages: {}", t);
    }
    if let Some(ref t) = targets.get_installed_applications {
        println!("  - getInstalledApplications: {}", t);
    }
    
    println!("\nTarget Library: {}", targets.get_target_library(version));
    println!("Framework Path: {}", android::get_framework_path(version));
    
    Ok(())
}

fn show_maps(args: &[String]) -> Result<()> {
    if args.is_empty() {
        println!("Error: pid required");
        return Ok(());
    }
    
    let pid: i32 = args[0].parse()?;
    println!("Memory maps for process {}", pid);
    
    let maps = process::parse_maps(pid)?;
    
    // Filter executable maps
    let exec_maps: Vec<_> = maps.iter().filter(|m| m.is_executable()).collect();
    
    println!("\nExecutable mappings: {}", exec_maps.len());
    for map in exec_maps {
        println!("  0x{:016x}-0x{:016x} {} {}",
            map.start, map.end, map.perms, map.pathname);
    }
    
    Ok(())
}
