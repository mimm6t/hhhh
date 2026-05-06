/// Symbol resolution for Android libraries
use crate::elf;
use crate::process::parse_maps;
use anyhow::{Context, Result};
use std::collections::HashMap;

pub struct SymbolResolver {
    cache: HashMap<String, Vec<elf::Symbol>>,
}

impl SymbolResolver {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
    
    pub fn resolve(&mut self, pid: i32, lib_name: &str, symbol_name: &str) -> Result<Option<u64>> {
        let maps = parse_maps(pid)?;
        let lib_map = maps.iter()
            .find(|m| m.pathname.contains(lib_name) && m.offset == 0)
            .context("Library not found")?;
        
        let base_addr = lib_map.start;
        let lib_path = &lib_map.pathname;
        
        let symbols = if let Some(cached) = self.cache.get(lib_path) {
            cached
        } else {
            let syms = elf::parse_symbols(lib_path)?;
            self.cache.insert(lib_path.clone(), syms);
            self.cache.get(lib_path).unwrap()
        };
        
        if let Some(sym) = elf::find_symbol(symbols, symbol_name) {
            Ok(Some(base_addr + sym.addr))
        } else {
            Ok(None)
        }
    }
    
    pub fn resolve_multiple(&mut self, pid: i32, lib_name: &str, names: &[&str]) -> Result<HashMap<String, u64>> {
        let mut result = HashMap::new();
        for name in names {
            if let Some(addr) = self.resolve(pid, lib_name, name)? {
                result.insert(name.to_string(), addr);
            }
        }
        Ok(result)
    }
}

pub fn find_method_by_pattern(pid: i32, lib_name: &str, pattern: &[u8]) -> Result<Option<u64>> {
    let maps = parse_maps(pid)?;
    
    for map in maps {
        if !map.pathname.contains(lib_name) || !map.is_executable() {
            continue;
        }
        
        let mem_path = format!("/proc/{}/mem", pid);
        let mut file = std::fs::File::open(&mem_path)?;
        use std::io::{Seek, SeekFrom, Read};
        
        file.seek(SeekFrom::Start(map.start))?;
        let size = (map.end - map.start) as usize;
        let mut buf = vec![0u8; size.min(1024 * 1024)];
        
        if file.read(&mut buf).is_ok() {
            for i in 0..buf.len().saturating_sub(pattern.len()) {
                if &buf[i..i + pattern.len()] == pattern {
                    return Ok(Some(map.start + i as u64));
                }
            }
        }
    }
    
    Ok(None)
}
