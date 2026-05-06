/// Android version detection and adaptation
use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidVersion {
    Android9,   // API 28
    Android10,  // API 29
    Android11,  // API 30
    Android12,  // API 31
    Android12L, // API 32
    Android13,  // API 33
    Android14,  // API 34
    Android15,  // API 35+
}

impl AndroidVersion {
    pub fn detect() -> Result<Self> {
        let output = Command::new("getprop")
            .arg("ro.build.version.sdk")
            .output()
            .context("Failed to execute getprop")?;
        
        let sdk_str = String::from_utf8_lossy(&output.stdout);
        let sdk: u32 = sdk_str.trim().parse()
            .context("Failed to parse SDK version")?;
        
        Ok(match sdk {
            28 => Self::Android9,
            29 => Self::Android10,
            30 => Self::Android11,
            31 => Self::Android12,
            32 => Self::Android12L,
            33 => Self::Android13,
            34 => Self::Android14,
            _ => Self::Android15,
        })
    }
    
    pub fn sdk_int(&self) -> u32 {
        match self {
            Self::Android9 => 28,
            Self::Android10 => 29,
            Self::Android11 => 30,
            Self::Android12 => 31,
            Self::Android12L => 32,
            Self::Android13 => 33,
            Self::Android14 => 34,
            Self::Android15 => 35,
        }
    }
}

pub struct PmsHookTargets {
    pub should_filter_application: Option<String>,
    pub get_packages_for_uid: Option<String>,
    pub get_installed_packages: Option<String>,
    pub get_installed_applications: Option<String>,
}

impl PmsHookTargets {
    pub fn for_version(version: AndroidVersion) -> Self {
        match version {
            AndroidVersion::Android14 | AndroidVersion::Android15 => Self {
                should_filter_application: Some("_ZN7android6server2pm14AppsFilterImpl24shouldFilterApplicationEP".to_string()),
                get_packages_for_uid: Some("getPackagesForUid".to_string()),
                get_installed_packages: None,
                get_installed_applications: None,
            },
            AndroidVersion::Android13 => Self {
                should_filter_application: Some("shouldFilterApplication".to_string()),
                get_packages_for_uid: Some("getPackagesForUid".to_string()),
                get_installed_packages: None,
                get_installed_applications: None,
            },
            AndroidVersion::Android11 | AndroidVersion::Android12 | AndroidVersion::Android12L => Self {
                should_filter_application: Some("shouldFilterApplication".to_string()),
                get_packages_for_uid: None,
                get_installed_packages: None,
                get_installed_applications: None,
            },
            AndroidVersion::Android9 | AndroidVersion::Android10 => Self {
                should_filter_application: None,
                get_packages_for_uid: None,
                get_installed_packages: Some("getInstalledPackages".to_string()),
                get_installed_applications: Some("getInstalledApplications".to_string()),
            },
        }
    }
    
    pub fn get_target_library(&self, version: AndroidVersion) -> &'static str {
        match version {
            AndroidVersion::Android14 | AndroidVersion::Android15 => "libandroid_servers.so",
            _ => "services.jar",
        }
    }
}

pub fn get_framework_path(version: AndroidVersion) -> &'static str {
    match version {
        AndroidVersion::Android14 | AndroidVersion::Android15 => "/system/lib64/libandroid_servers.so",
        _ => "/system/framework/services.jar",
    }
}
