/// Hook engine for PMS (PackageManagerService)
/// 
/// Implements hooking strategy for hiding app list

use crate::config::Config;
use crate::process::find_process_by_name;
use crate::pms_hook::PmsHook;
use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};

/// PMS Hook Engine
pub struct PmsHookEngine {
    config: Arc<Mutex<Config>>,
    system_server_pid: Option<i32>,
    pms_hook: Option<PmsHook>,
}

impl PmsHookEngine {
    /// Create new hook engine
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            system_server_pid: None,
            pms_hook: None,
        }
    }
    
    /// Initialize hook engine
    pub fn init(&mut self) -> Result<()> {
        self.system_server_pid = find_process_by_name("system_server")?;
        
        if self.system_server_pid.is_none() {
            anyhow::bail!("system_server process not found");
        }
        
        log::info!("Found system_server PID: {:?}", self.system_server_pid);
        Ok(())
    }
    
    /// Install hooks
    pub fn install_hooks(&mut self) -> Result<()> {
        let pid = self.system_server_pid.context("system_server not initialized")?;
        let config = self.config.lock().unwrap().clone();
        
        log::info!("Installing hooks for PID {}", pid);
        
        let mut pms_hook = PmsHook::new(pid, config)?;
        pms_hook.install()?;
        
        self.pms_hook = Some(pms_hook);
        
        Ok(())
    }
    
    /// Update configuration
    pub fn update_config(&mut self, new_config: Config) {
        let mut config = self.config.lock().unwrap();
        *config = new_config;
        log::info!("Configuration updated");
    }
    
    /// Uninstall all hooks
    pub fn uninstall_hooks(&mut self) -> Result<()> {
        if let Some(mut hook) = self.pms_hook.take() {
            hook.uninstall()?;
        }
        Ok(())
    }
}

impl Drop for PmsHookEngine {
    fn drop(&mut self) {
        let _ = self.uninstall_hooks();
    }
}
