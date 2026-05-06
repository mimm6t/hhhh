/// Configuration management for Hide-My-Applist
/// 
/// Manages app hiding rules, templates, and scope configuration

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// Application configuration for hiding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Use whitelist mode (hide all except listed)
    pub use_whitelist: bool,
    
    /// Exclude system apps from hiding
    pub exclude_system_apps: bool,
    
    /// Extra app list (blacklist or whitelist depending on mode)
    pub extra_app_list: HashSet<String>,
    
    /// Template names to apply
    pub apply_templates: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            use_whitelist: false,
            exclude_system_apps: true,
            extra_app_list: HashSet::new(),
            apply_templates: Vec::new(),
        }
    }
}

/// Template for app hiding rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Template name
    pub name: String,
    
    /// Apps in this template
    pub app_list: HashSet<String>,
}

/// Main configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration version
    pub config_version: u32,
    
    /// Enable detailed logging
    pub detail_log: bool,
    
    /// Maximum log size in KB
    pub max_log_size: usize,
    
    /// Scope: package name -> app config
    pub scope: HashMap<String, AppConfig>,
    
    /// Templates
    pub templates: HashMap<String, Template>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: 1,
            detail_log: false,
            max_log_size: 1024,
            scope: HashMap::new(),
            templates: HashMap::new(),
        }
    }
}

impl Config {
    /// Load configuration from JSON file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;
        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config JSON")?;
        Ok(config)
    }
    
    /// Save configuration to JSON file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(path.as_ref(), json)
            .context("Failed to write config file")?;
        Ok(())
    }
    
    /// Check if hook is enabled for a package
    pub fn is_hook_enabled(&self, package: &str) -> bool {
        self.scope.contains_key(package)
    }
    
    /// Determine if target app should be hidden from caller
    pub fn should_hide(&self, caller: &str, target: &str, system_apps: &HashSet<String>) -> bool {
        // Don't hide from self
        if caller == target {
            return false;
        }
        
        // Get caller's config
        let app_config = match self.scope.get(caller) {
            Some(cfg) => cfg,
            None => return false,
        };
        
        // Check if target is system app and should be excluded
        if app_config.use_whitelist && app_config.exclude_system_apps && system_apps.contains(target) {
            return false;
        }
        
        // Check extra app list
        if app_config.extra_app_list.contains(target) {
            return !app_config.use_whitelist;
        }
        
        // Check templates
        for template_name in &app_config.apply_templates {
            if let Some(template) = self.templates.get(template_name) {
                if template.app_list.contains(target) {
                    return !app_config.use_whitelist;
                }
            }
        }
        
        // Default: hide if whitelist mode, show if blacklist mode
        app_config.use_whitelist
    }
}

/// Packages that should never be hidden
pub const PACKAGES_SHOULD_NOT_HIDE: &[&str] = &[
    "android",
    "com.android.systemui",
];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_should_hide_blacklist() {
        let mut config = Config::default();
        let mut app_config = AppConfig::default();
        app_config.use_whitelist = false;
        app_config.extra_app_list.insert("com.example.hidden".to_string());
        config.scope.insert("com.example.caller".to_string(), app_config);
        
        let system_apps = HashSet::new();
        
        assert!(config.should_hide("com.example.caller", "com.example.hidden", &system_apps));
        assert!(!config.should_hide("com.example.caller", "com.example.visible", &system_apps));
    }
    
    #[test]
    fn test_should_hide_whitelist() {
        let mut config = Config::default();
        let mut app_config = AppConfig::default();
        app_config.use_whitelist = true;
        app_config.extra_app_list.insert("com.example.visible".to_string());
        config.scope.insert("com.example.caller".to_string(), app_config);
        
        let system_apps = HashSet::new();
        
        assert!(!config.should_hide("com.example.caller", "com.example.visible", &system_apps));
        assert!(config.should_hide("com.example.caller", "com.example.hidden", &system_apps));
    }
}
