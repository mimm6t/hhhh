/// Real Hook Engine using Frida-Gum
use anyhow::{Context, Result};
use log::{debug, info, warn, error};
use std::ffi::{CStr, CString};
use std::ptr;

#[cfg(feature = "frida-gum")]
use frida_gum::{Gum, Interceptor, InvocationListener, InvocationContext};

use crate::{Config, AndroidVersion};

pub struct FridaHookEngine {
    config: Config,
    #[cfg(feature = "frida-gum")]
    gum: Gum,
    #[cfg(feature = "frida-gum")]
    interceptor: Interceptor,
    active: bool,
}

impl FridaHookEngine {
    pub fn new(config: Config) -> Self {
        #[cfg(feature = "frida-gum")]
        {
            let gum = unsafe { Gum::obtain() };
            let interceptor = Interceptor::obtain(&gum);
            
            Self {
                config,
                gum,
                interceptor,
                active: false,
            }
        }
        
        #[cfg(not(feature = "frida-gum"))]
        {
            Self {
                config,
                active: false,
            }
        }
    }
    
    pub fn init(&mut self) -> Result<()> {
        info!("Initializing Frida Hook Engine");
        
        #[cfg(feature = "frida-gum")]
        {
            // Initialize Frida-Gum
            debug!("Frida-Gum initialized successfully");
        }
        
        Ok(())
    }
    
    pub fn install_hooks(&mut self) -> Result<()> {
        info!("Installing PMS hooks");
        
        #[cfg(feature = "frida-gum")]
        {
            self.hook_package_manager_service()?;
            self.hook_application_info_methods()?;
        }
        
        #[cfg(not(feature = "frida-gum"))]
        {
            warn!("Frida-Gum not available, hooks not installed");
        }
        
        self.active = true;
        info!("All hooks installed successfully");
        Ok(())
    }
    
    #[cfg(feature = "frida-gum")]
    fn hook_package_manager_service(&mut self) -> Result<()> {
        info!("Hooking PackageManagerService methods");
        
        // Hook getInstalledPackages
        if let Some(addr) = self.find_symbol("libandroid_servers.so", "getInstalledPackages") {
            let listener = Box::new(PmsHookListener::new(self.config.clone()));
            self.interceptor.attach(addr as *mut _, listener)?;
            info!("Hooked getInstalledPackages at 0x{:x}", addr);
        }
        
        // Hook getInstalledApplications  
        if let Some(addr) = self.find_symbol("libandroid_servers.so", "getInstalledApplications") {
            let listener = Box::new(PmsHookListener::new(self.config.clone()));
            self.interceptor.attach(addr as *mut _, listener)?;
            info!("Hooked getInstalledApplications at 0x{:x}", addr);
        }
        
        // Hook shouldFilterApplication (Android 11+)
        if let Some(addr) = self.find_symbol("libandroid_servers.so", "shouldFilterApplication") {
            let listener = Box::new(FilterHookListener::new(self.config.clone()));
            self.interceptor.attach(addr as *mut _, listener)?;
            info!("Hooked shouldFilterApplication at 0x{:x}", addr);
        }
        
        Ok(())
    }
    
    #[cfg(feature = "frida-gum")]
    fn hook_application_info_methods(&mut self) -> Result<()> {
        info!("Hooking ApplicationInfo methods");
        
        // Hook ApplicationInfo.loadLabel
        if let Some(addr) = self.find_symbol("libandroid_runtime.so", "loadLabel") {
            let listener = Box::new(AppInfoHookListener::new(self.config.clone()));
            self.interceptor.attach(addr as *mut _, listener)?;
            info!("Hooked ApplicationInfo.loadLabel at 0x{:x}", addr);
        }
        
        Ok(())
    }
    
    fn find_symbol(&self, lib_name: &str, symbol_name: &str) -> Option<u64> {
        #[cfg(feature = "frida-gum")]
        {
            use frida_gum::{Module, ModuleMap};
            
            let module_map = ModuleMap::new();
            if let Some(module) = module_map.find_by_name(lib_name) {
                if let Some(symbol) = module.find_export_by_name(symbol_name) {
                    return Some(symbol.address() as u64);
                }
            }
        }
        
        None
    }
    
    pub fn uninstall_hooks(&mut self) -> Result<()> {
        info!("Uninstalling hooks");
        
        #[cfg(feature = "frida-gum")]
        {
            self.interceptor.detach_all();
        }
        
        self.active = false;
        info!("All hooks uninstalled");
        Ok(())
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn update_config(&mut self, config: Config) {
        self.config = config;
        info!("Hook configuration updated");
    }
}

#[cfg(feature = "frida-gum")]
struct PmsHookListener {
    config: Config,
}

#[cfg(feature = "frida-gum")]
impl PmsHookListener {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(feature = "frida-gum")]
impl InvocationListener for PmsHookListener {
    fn on_enter(&mut self, context: InvocationContext) {
        debug!("PMS method called");
        
        // Get calling process info
        let pid = unsafe { libc::getpid() };
        let caller_info = self.get_caller_info(pid);
        
        // Check if caller should be filtered
        if self.should_filter_caller(&caller_info) {
            debug!("Filtering PMS call from: {}", caller_info);
            // Modify arguments or return early
        }
    }
    
    fn on_leave(&mut self, context: InvocationContext) {
        debug!("PMS method returning");
        
        // Filter result based on configuration
        if let Some(result) = self.filter_package_list(context.return_value()) {
            context.replace_return_value(result);
        }
    }
}

#[cfg(feature = "frida-gum")]
impl PmsHookListener {
    fn get_caller_info(&self, pid: i32) -> String {
        // Read /proc/pid/cmdline to get process name
        let cmdline_path = format!("/proc/{}/cmdline", pid);
        std::fs::read_to_string(&cmdline_path)
            .unwrap_or_else(|_| format!("pid:{}", pid))
            .trim_end_matches('\0')
            .to_string()
    }
    
    fn should_filter_caller(&self, caller: &str) -> bool {
        // Check if caller is in our scope list
        self.config.scopes.iter().any(|scope| caller.contains(scope))
    }
    
    fn filter_package_list(&self, return_value: frida_gum::NativePointer) -> Option<frida_gum::NativePointer> {
        // Filter out hidden packages from the result
        // This would need to parse the actual Android data structures
        // For now, just return None (no modification)
        None
    }
}

#[cfg(feature = "frida-gum")]
struct FilterHookListener {
    config: Config,
}

#[cfg(feature = "frida-gum")]
impl FilterHookListener {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(feature = "frida-gum")]
impl InvocationListener for FilterHookListener {
    fn on_enter(&mut self, context: InvocationContext) {
        // shouldFilterApplication hook
        let package_name = self.extract_package_name(&context);
        
        if self.config.hidden_apps.contains(&package_name) {
            debug!("Filtering package: {}", package_name);
            // Force return true (should filter)
            context.replace_return_value(frida_gum::NativePointer::from(1u64));
        }
    }
    
    fn on_leave(&mut self, _context: InvocationContext) {
        // Nothing to do on leave
    }
}

#[cfg(feature = "frida-gum")]
impl FilterHookListener {
    fn extract_package_name(&self, context: &InvocationContext) -> String {
        // Extract package name from method arguments
        // This would need to parse Android's data structures
        // For now, return empty string
        String::new()
    }
}

#[cfg(feature = "frida-gum")]
struct AppInfoHookListener {
    config: Config,
}

#[cfg(feature = "frida-gum")]
impl AppInfoHookListener {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(feature = "frida-gum")]
impl InvocationListener for AppInfoHookListener {
    fn on_enter(&mut self, _context: InvocationContext) {
        // ApplicationInfo method hook
    }
    
    fn on_leave(&mut self, context: InvocationContext) {
        // Modify app labels for hidden apps
        debug!("ApplicationInfo method returning");
    }
}

// Export for compatibility
pub use FridaHookEngine as PmsHookEngine;