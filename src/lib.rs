/// Hide-My-Applist Rust Implementation
/// 
/// A kernel-level app hiding solution using wxshadow + rustFrida
/// 
/// This is a rewrite of Hide-My-Applist using wxshadow kernel module
/// for stealthy hooking instead of Xposed framework.

pub mod wxshadow;
pub mod config;
pub mod process;
pub mod hook;
pub mod advanced_hook;
pub mod elf;
pub mod symbol;
pub mod android;
pub mod pms_hook;

#[cfg(feature = "android")]
pub mod jni;

pub use config::Config;
pub use hook::PmsHookEngine;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging
pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
