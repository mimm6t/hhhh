/// Hide-My-Applist Rust - Real Frida-based Hook Implementation
pub mod android;
pub mod config;
pub mod frida_hook;

#[cfg(feature = "android")]
pub mod jni;

// Re-export main types
pub use android::AndroidVersion;
pub use config::Config;
pub use frida_hook::FridaHookEngine as PmsHookEngine;

#[cfg(target_os = "android")]
use android_logger::{Config as LogConfig, FilterBuilder};

/// Initialize logging for Android
#[cfg(target_os = "android")]
pub fn init_logging() {
    android_logger::init_once(
        LogConfig::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("HMA-Rust")
            .with_filter(FilterBuilder::new().parse("debug").build()),
    );
}

/// Initialize logging for desktop
#[cfg(not(target_os = "android"))]
pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
}