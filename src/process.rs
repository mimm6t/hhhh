/// Process and memory utilities
/// 
/// Provides utilities for process inspection and memory mapping

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Memory mapping entry
#[derive(Debug, Clone)]
pub struct MemoryMap {
    pub start: u64,
    pub end: u64,
    pub perms: String,
    pub offset: u64,
    pub dev: String,
    pub inode: u64,
    pub pathname: String,
}

impl MemoryMap {
    /// Check if this mapping is executable
    pub fn is_executable(&self) -> bool {
        self.perms.contains('x')
    }
    
    /// Check if this mapping is readable
    pub fn is_readable(&self) -> bool {
        self.perms.contains('r')
    }
    
    /// Get size of this mapping
    pub fn size(&self) -> u64 {
        self.end - self.start
    }
}

/// Parse /proc/[pid]/maps
pub fn parse_maps(pid: i32) -> Result<Vec<MemoryMap>> {
    let maps_path = format!("/proc/{}/maps", pid);
    let content = fs::read_to_string(&maps_path)
        .context(format!("Failed to read {}", maps_path))?;
    
    let mut maps = Vec::new();
    for line in content.lines() {
        if let Some(map) = parse_map_line(line) {
            maps.push(map);
        }
    }
    
    Ok(maps)
}

fn parse_map_line(line: &str) -> Option<MemoryMap> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }
    
    let addr_range: Vec<&str> = parts[0].split('-').collect();
    if addr_range.len() != 2 {
        return None;
    }
    
    let start = u64::from_str_radix(addr_range[0], 16).ok()?;
    let end = u64::from_str_radix(addr_range[1], 16).ok()?;
    let perms = parts[1].to_string();
    let offset = u64::from_str_radix(parts[2], 16).ok()?;
    let dev = parts[3].to_string();
    let inode = parts[4].parse().ok()?;
    let pathname = if parts.len() > 5 {
        parts[5..].join(" ")
    } else {
        String::new()
    };
    
    Some(MemoryMap {
        start,
        end,
        perms,
        offset,
        dev,
        inode,
        pathname,
    })
}

/// Find executable mappings for a library
pub fn find_library_executable_maps(pid: i32, lib_name: &str) -> Result<Vec<MemoryMap>> {
    let maps = parse_maps(pid)?;
    Ok(maps.into_iter()
        .filter(|m| m.is_executable() && m.pathname.contains(lib_name))
        .collect())
}

/// Get process name from /proc/[pid]/cmdline
pub fn get_process_name(pid: i32) -> Result<String> {
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    let content = fs::read_to_string(&cmdline_path)
        .context(format!("Failed to read {}", cmdline_path))?;
    
    // cmdline is null-terminated
    let name = content.split('\0').next().unwrap_or("");
    Ok(name.to_string())
}

/// Find process ID by name
pub fn find_process_by_name(name: &str) -> Result<Option<i32>> {
    let proc_dir = Path::new("/proc");
    
    for entry in fs::read_dir(proc_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        // Check if directory name is a number (PID)
        if let Ok(pid) = file_name_str.parse::<i32>() {
            if let Ok(proc_name) = get_process_name(pid) {
                if proc_name.contains(name) {
                    return Ok(Some(pid));
                }
            }
        }
    }
    
    Ok(None)
}

/// Calculate offset from library base
pub fn calculate_offset(maps: &[MemoryMap], absolute_addr: u64) -> Option<(String, u64)> {
    for map in maps {
        if absolute_addr >= map.start && absolute_addr < map.end {
            let offset = absolute_addr - map.start + map.offset;
            return Some((map.pathname.clone(), offset));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_map_line() {
        let line = "7f1234567000-7f1234568000 r-xp 00001000 08:01 12345 /system/lib64/libc.so";
        let map = parse_map_line(line).unwrap();
        
        assert_eq!(map.start, 0x7f1234567000);
        assert_eq!(map.end, 0x7f1234568000);
        assert_eq!(map.perms, "r-xp");
        assert!(map.is_executable());
        assert!(map.is_readable());
        assert_eq!(map.pathname, "/system/lib64/libc.so");
    }
}
