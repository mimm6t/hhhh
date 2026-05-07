/// Configuration management for Hide-My-Applist
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// List of package names to hide
    pub hidden_apps: HashSet<String>,
    
    /// List of process names that should see filtered results
    pub scopes: HashSet<String>,
    
    /// Templates for quick configuration
    pub templates: Vec<Template>,
    
    /// Whether to enable verbose logging
    pub verbose_logging: bool,
    
    /// Whether to hide system apps
    pub hide_system_apps: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub hidden_apps: HashSet<String>,
    pub scopes: HashSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut scopes = HashSet::new();
        // Default scopes - common apps that check for other apps
        scopes.insert("com.android.vending".to_string()); // Google Play Store
        scopes.insert("com.google.android.gms".to_string()); // Google Play Services
        scopes.insert("com.android.packageinstaller".to_string()); // Package Installer
        
        Self {
            hidden_apps: HashSet::new(),
            scopes,
            templates: Vec::new(),
            verbose_logging: false,
            hide_system_apps: false,
        }
    }
}

impl Config {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add an app to the hidden list
    pub fn hide_app(&mut self, package_name: String) {
        self.hidden_apps.insert(package_name);
    }
    
    /// Remove an app from the hidden list
    pub fn show_app(&mut self, package_name: &str) {
        self.hidden_apps.remove(package_name);
    }
    
    /// Check if an app is hidden
    pub fn is_app_hidden(&self, package_name: &str) -> bool {
        self.hidden_apps.contains(package_name)
    }
    
    /// Add a scope (process that should see filtered results)
    pub fn add_scope(&mut self, process_name: String) {
        self.scopes.insert(process_name);
    }
    
    /// Remove a scope
    pub fn remove_scope(&mut self, process_name: &str) {
        self.scopes.remove(process_name);
    }
    
    /// Check if a process is in scope
    pub fn is_in_scope(&self, process_name: &str) -> bool {
        self.scopes.iter().any(|scope| process_name.contains(scope))
    }
    
    /// Add a template
    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
    }
    
    /// Remove a template by name
    pub fn remove_template(&mut self, name: &str) {
        self.templates.retain(|t| t.name != name);
    }
    
    /// Apply a template to current config
    pub fn apply_template(&mut self, template_name: &str) -> Result<(), String> {
        if let Some(template) = self.templates.iter().find(|t| t.name == template_name) {
            self.hidden_apps.extend(template.hidden_apps.clone());
            self.scopes.extend(template.scopes.clone());
            Ok(())
        } else {
            Err(format!("Template '{}' not found", template_name))
        }
    }
    
    /// Get default templates
    pub fn get_default_templates() -> Vec<Template> {
        vec![
            Template {
                name: "Banking Apps".to_string(),
                description: "Hide from banking and financial apps".to_string(),
                hidden_apps: [
                    "com.topjohnwu.magisk",
                    "com.android.shell",
                    "eu.chainfire.supersu",
                ].iter().map(|s| s.to_string()).collect(),
                scopes: [
                    "com.chase.sig.android",
                    "com.bankofamerica.angelapp",
                    "com.paypal.android.p2pmobile",
                ].iter().map(|s| s.to_string()).collect(),
            },
            Template {
                name: "Gaming Apps".to_string(),
                description: "Hide from gaming and anti-cheat apps".to_string(),
                hidden_apps: [
                    "com.topjohnwu.magisk",
                    "de.robv.android.xposed.installer",
                    "com.android.shell",
                ].iter().map(|s| s.to_string()).collect(),
                scopes: [
                    "com.miHoYo.GenshinImpact",
                    "com.pubg.imobile",
                    "com.tencent.ig",
                ].iter().map(|s| s.to_string()).collect(),
            },
            Template {
                name: "Work Profile".to_string(),
                description: "Hide from work and enterprise apps".to_string(),
                hidden_apps: [
                    "com.topjohnwu.magisk",
                    "com.android.shell",
                    "eu.chainfire.supersu",
                ].iter().map(|s| s.to_string()).collect(),
                scopes: [
                    "com.microsoft.office.outlook",
                    "com.slack",
                    "com.google.android.apps.work.clouddpc",
                ].iter().map(|s| s.to_string()).collect(),
            },
        ]
    }
    
    /// Load default templates into config
    pub fn load_default_templates(&mut self) {
        self.templates = Self::get_default_templates();
    }
    
    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}