/// Actual PMS Hook implementation
use crate::android::{AndroidVersion, PmsHookTargets};
use crate::config::Config;
use crate::symbol::SymbolResolver;
use crate::wxshadow::{self, arm64};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct PmsHook {
    pid: i32,
    version: AndroidVersion,
    config: Arc<Mutex<Config>>,
    system_apps: HashSet<String>,
    hooks: Vec<HookPoint>,
    resolver: SymbolResolver,
}

struct HookPoint {
    name: String,
    addr: u64,
}

impl PmsHook {
    pub fn new(pid: i32, config: Config) -> Result<Self> {
        let version = AndroidVersion::detect()?;
        log::info!("Detected Android version: {:?} (SDK {})", version, version.sdk_int());
        
        Ok(Self {
            pid,
            version,
            config: Arc::new(Mutex::new(config)),
            system_apps: HashSet::new(),
            hooks: Vec::new(),
            resolver: SymbolResolver::new(),
        })
    }
    
    pub fn install(&mut self) -> Result<()> {
        let targets = PmsHookTargets::for_version(self.version);
        
        match self.version {
            AndroidVersion::Android14 | AndroidVersion::Android15 => {
                self.hook_android14()?;
            }
            AndroidVersion::Android13 => {
                self.hook_android13()?;
            }
            AndroidVersion::Android11 | AndroidVersion::Android12 | AndroidVersion::Android12L => {
                self.hook_android11()?;
            }
            AndroidVersion::Android9 | AndroidVersion::Android10 => {
                self.hook_android9()?;
            }
        }
        
        log::info!("Installed {} hooks", self.hooks.len());
        Ok(())
    }
    
    fn hook_android14(&mut self) -> Result<()> {
        // Hook shouldFilterApplication in libandroid_servers.so
        let addr = self.resolver.resolve(
            self.pid,
            "libandroid_servers.so",
            "shouldFilterApplication"
        )?.context("shouldFilterApplication not found")?;
        
        log::info!("Found shouldFilterApplication at 0x{:x}", addr);
        
        // Generate hook: return false (don't filter)
        let patch = [
            arm64::mov_x0_imm(0).to_le_bytes(),  // mov x0, #0
            arm64::RET.to_le_bytes(),             // ret
        ].concat();
        
        wxshadow::write_patch(self.pid, addr, &patch)?;
        
        self.hooks.push(HookPoint {
            name: "shouldFilterApplication".to_string(),
            addr,
        });
        
        Ok(())
    }
    
    fn hook_android13(&mut self) -> Result<()> {
        // Similar to Android 14 but different symbol mangling
        let addr = self.resolver.resolve(
            self.pid,
            "libandroid_servers.so",
            "shouldFilterApplication"
        )?.context("shouldFilterApplication not found")?;
        
        let patch = [
            arm64::mov_x0_imm(0).to_le_bytes(),
            arm64::RET.to_le_bytes(),
        ].concat();
        
        wxshadow::write_patch(self.pid, addr, &patch)?;
        
        self.hooks.push(HookPoint {
            name: "shouldFilterApplication".to_string(),
            addr,
        });
        
        Ok(())
    }
    
    fn hook_android11(&mut self) -> Result<()> {
        // Android 11-12: Hook in framework
        let addr = self.resolver.resolve(
            self.pid,
            "libandroid_runtime.so",
            "shouldFilterApplication"
        )?.context("shouldFilterApplication not found")?;
        
        let patch = [
            arm64::mov_x0_imm(0).to_le_bytes(),
            arm64::RET.to_le_bytes(),
        ].concat();
        
        wxshadow::write_patch(self.pid, addr, &patch)?;
        
        self.hooks.push(HookPoint {
            name: "shouldFilterApplication".to_string(),
            addr,
        });
        
        Ok(())
    }
    
    fn hook_android9(&mut self) -> Result<()> {
        // Android 9-10: Hook getInstalledPackages/getInstalledApplications
        // These are Java methods, need different approach
        log::warn!("Android 9-10 requires Java hook, not yet implemented");
        Ok(())
    }
    
    pub fn uninstall(&mut self) -> Result<()> {
        for hook in &self.hooks {
            log::info!("Removing hook: {} at 0x{:x}", hook.name, hook.addr);
            wxshadow::release_shadow(self.pid, hook.addr)?;
        }
        self.hooks.clear();
        Ok(())
    }
    
    pub fn should_hide(&self, caller: &str, target: &str) -> bool {
        let config = self.config.lock().unwrap();
        config.should_hide(caller, target, &self.system_apps)
    }
}

impl Drop for PmsHook {
    fn drop(&mut self) {
        let _ = self.uninstall();
    }
}

// Advanced hook with callback support
pub struct CallbackHook {
    pid: i32,
    addr: u64,
    callback_addr: u64,
}

impl CallbackHook {
    pub fn new(pid: i32, target_addr: u64) -> Self {
        Self {
            pid,
            addr: target_addr,
            callback_addr: 0,
        }
    }
    
    pub fn install_with_callback(&mut self, callback: extern "C" fn() -> i32) -> Result<()> {
        self.callback_addr = callback as u64;
        
        // Generate trampoline that calls callback
        let mut code = Vec::new();
        
        // Save context
        code.extend_from_slice(&0xa9bf7bfdu32.to_le_bytes()); // stp x29, x30, [sp, #-16]!
        code.extend_from_slice(&0xa9bf07e0u32.to_le_bytes()); // stp x0, x1, [sp, #-16]!
        
        // Load callback address into x16
        // This is simplified - real implementation needs proper address loading
        
        // Call callback
        code.extend_from_slice(&0xd63f0200u32.to_le_bytes()); // blr x16
        
        // Restore context
        code.extend_from_slice(&0xa8c107e0u32.to_le_bytes()); // ldp x0, x1, [sp], #16
        code.extend_from_slice(&0xa8c17bfdu32.to_le_bytes()); // ldp x29, x30, [sp], #16
        
        // Return
        code.extend_from_slice(&arm64::RET.to_le_bytes());
        
        wxshadow::write_patch(self.pid, self.addr, &code)?;
        
        Ok(())
    }
}
