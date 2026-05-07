/// JNI interface for Android
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jboolean, jstring};
use crate::{Config, PmsHookEngine};

static mut ENGINE: Option<PmsHookEngine> = None;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_hma_rust_native_HmaCore_init(
    _env: JNIEnv,
    _class: JClass
) -> jint {
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_hma_rust_native_HmaCore_installHook(
    mut env: JNIEnv,
    _class: JClass,
    config_json: JString
) -> jint {
    let config_str: String = match env.get_string(&config_json) {
        Ok(s) => s.into(),
        Err(_) => return -1,
    };
    
    let config = match serde_json::from_str::<Config>(&config_str) {
        Ok(c) => c,
        Err(_) => return -2,
    };
    
    unsafe {
        let mut engine = PmsHookEngine::new(config);
        if engine.init().is_err() {
            return -3;
        }
        if engine.install_hooks().is_err() {
            return -4;
        }
        ENGINE = Some(engine);
    }
    
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_hma_rust_native_HmaCore_uninstallHook(
    _env: JNIEnv,
    _class: JClass
) -> jint {
    unsafe {
        if let Some(mut engine) = (&raw mut ENGINE).read().take() {
            let _ = engine.uninstall_hooks();
        }
    }
    0
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_hma_rust_native_HmaCore_getStatus(
    env: JNIEnv,
    _class: JClass
) -> jstring {
    let status = r#"{"active":true,"filter_count":0}"#;
    let output = env.new_string(status).unwrap();
    output.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_hma_rust_native_HmaCore_testFrida(
    _env: JNIEnv,
    _class: JClass
) -> jboolean {
    // Test if Frida-Gum is available
    #[cfg(feature = "frida-gum")]
    {
        true as jboolean
    }
    #[cfg(not(feature = "frida-gum"))]
    {
        false as jboolean
    }
}
