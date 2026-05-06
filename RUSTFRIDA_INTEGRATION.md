# rustFrida 集成指南

本文档说明如何将 Hide-My-Applist Rust 与 rustFrida 集成，实现更强大的 Hook 能力。

## rustFrida 简介

rustFrida 是基于 Frida 的 Rust 实现，提供：
- Java Hook：Hook Java 方法
- Native Hook：Hook Native 函数
- 内存操作：读写进程内存
- 脚本注入：注入 JavaScript 脚本

## 集成架构

```
┌─────────────────────────────────────────┐
│     Hide-My-Applist Rust                │
│  ┌───────────────────────────────────┐  │
│  │  rustFrida 集成层                  │  │
│  │  - Java Hook 封装                  │  │
│  │  - Native Hook 封装                │  │
│  │  - 脚本管理                        │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  wxshadow 隐藏层                   │  │
│  │  - Hook 痕迹隐藏                   │  │
│  │  - 内存保护                        │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
         ↓                    ↓
    rustFrida              wxshadow.kpm
```

## 实现方案

### 方案 1：rustFrida + wxshadow 混合模式

**优势：**
- 利用 rustFrida 的高级功能
- 使用 wxshadow 隐藏 Hook 痕迹
- 最佳的功能性和隐蔽性

**实现步骤：**

1. **使用 rustFrida 定位目标**
```rust
use frida::Frida;

// 连接到 system_server
let device = frida.get_local_device()?;
let session = device.attach("system_server")?;

// 查找 PMS 类
let script = session.create_script(r#"
    Java.perform(function() {
        var AppsFilterImpl = Java.use("com.android.server.pm.AppsFilterImpl");
        var method = AppsFilterImpl.shouldFilterApplication;
        
        // 获取方法地址
        send({
            type: "method_address",
            address: method.implementation.toString()
        });
    });
"#)?;
```

2. **使用 wxshadow 设置 Hook**
```rust
// 从 rustFrida 获取地址
let method_addr = get_method_address_from_frida()?;

// 使用 wxshadow 设置无痕 Hook
wxshadow::write_patch(pid, method_addr, &hook_code)?;
```

3. **实现 Hook 回调**
```rust
// Hook 回调在内核层触发
// 通过共享内存或其他 IPC 机制与用户态通信
```

### 方案 2：纯 wxshadow 模式（当前实现）

**优势：**
- 不依赖 rustFrida
- 更轻量级
- 完全内核层实现

**劣势：**
- 需要手动定位地址
- 功能相对有限

### 方案 3：纯 rustFrida 模式

**优势：**
- 功能强大
- 易于开发和调试

**劣势：**
- Hook 痕迹明显
- 容易被检测

## rustFrida Hook 示例

### Java Hook 示例

```rust
use frida::{Frida, ScriptOption};

pub struct JavaHook {
    frida: Frida,
    session: Session,
}

impl JavaHook {
    pub fn new() -> Result<Self> {
        let frida = Frida::obtain();
        let device = frida.get_local_device()?;
        let session = device.attach("system_server")?;
        
        Ok(Self { frida, session })
    }
    
    pub fn hook_should_filter_application(&self) -> Result<()> {
        let script = self.session.create_script(r#"
            Java.perform(function() {
                var AppsFilterImpl = Java.use("com.android.server.pm.AppsFilterImpl");
                
                AppsFilterImpl.shouldFilterApplication.implementation = function(
                    snapshot, callingUid, callingSetting, targetPkgSetting, userId
                ) {
                    // 获取调用者包名
                    var Computer = Java.use("com.android.server.pm.Computer");
                    var callingApps = Computer.getPackagesForUid.call(snapshot, callingUid);
                    
                    if (callingApps != null && callingApps.length > 0) {
                        var caller = callingApps[0];
                        var target = targetPkgSetting.name.value;
                        
                        // 调用 Rust 层判断是否隐藏
                        var shouldHide = this.shouldHideApp(caller, target);
                        
                        if (shouldHide) {
                            console.log("[HMA] Hiding " + target + " from " + caller);
                            return true;
                        }
                    }
                    
                    // 调用原始方法
                    return this.shouldFilterApplication(
                        snapshot, callingUid, callingSetting, targetPkgSetting, userId
                    );
                };
            });
        "#)?;
        
        script.load()?;
        Ok(())
    }
    
    // 从 Rust 层调用
    fn should_hide_app(&self, caller: &str, target: &str) -> bool {
        // 调用配置管理模块
        let config = self.config.lock().unwrap();
        config.should_hide(caller, target, &self.system_apps)
    }
}
```

### Native Hook 示例

```rust
pub struct NativeHook {
    session: Session,
}

impl NativeHook {
    pub fn hook_binder_transaction(&self) -> Result<()> {
        let script = self.session.create_script(r#"
            // Hook libbinder.so 中的 Binder 事务
            var libbinder = Process.getModuleByName("libbinder.so");
            var transact = libbinder.getExportByName("_ZN7android14BpBinder8transactEjRKNS_6ParcelEPS1_j");
            
            Interceptor.attach(transact, {
                onEnter: function(args) {
                    // args[0] = this
                    // args[1] = code
                    // args[2] = data (Parcel)
                    // args[3] = reply (Parcel)
                    
                    var code = args[1].toInt32();
                    
                    // 检查是否是 PMS 相关的事务
                    if (this.isPmsTransaction(code)) {
                        console.log("[HMA] Intercepting PMS transaction: " + code);
                        
                        // 读取 Parcel 数据
                        var data = this.readParcel(args[2]);
                        
                        // 修改数据
                        this.modifyParcel(args[2], data);
                    }
                },
                onLeave: function(retval) {
                    // 处理返回值
                }
            });
        "#)?;
        
        script.load()?;
        Ok(())
    }
}
```

## wxshadow 隐藏 rustFrida Hook

### 问题

rustFrida 的 Hook 会修改内存，容易被检测：
- Inline Hook 修改指令
- Trampoline 代码可见
- Hook 表可被扫描

### 解决方案

使用 wxshadow 隐藏 rustFrida 的 Hook 痕迹：

```rust
pub struct StealthHook {
    frida_hook: JavaHook,
    wxshadow_manager: HookManager,
}

impl StealthHook {
    pub fn install_stealth_hook(&mut self, target_addr: u64) -> Result<()> {
        // 1. 使用 rustFrida 获取目标信息
        let method_info = self.frida_hook.get_method_info()?;
        
        // 2. 生成 Hook 代码
        let hook_code = self.generate_hook_code(&method_info)?;
        
        // 3. 使用 wxshadow 写入 Hook
        wxshadow::write_patch(self.pid, target_addr, &hook_code)?;
        
        // 4. 设置 Hook 回调
        self.setup_hook_callback(target_addr)?;
        
        Ok(())
    }
    
    fn generate_hook_code(&self, info: &MethodInfo) -> Result<Vec<u8>> {
        // 生成跳转到 Hook 处理函数的代码
        let mut code = Vec::new();
        
        // 保存上下文
        code.extend_from_slice(&arm64::stp_x29_x30_sp_pre(-16).to_le_bytes());
        
        // 调用 Hook 处理函数
        // ...
        
        // 恢复上下文
        code.extend_from_slice(&arm64::ldp_x29_x30_sp_post(16).to_le_bytes());
        
        // 返回
        code.extend_from_slice(&arm64::RET.to_le_bytes());
        
        Ok(code)
    }
}
```

## 通信机制

### 内核态 ↔ 用户态通信

**方案 1：共享内存**
```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SharedState {
    should_hide: Arc<AtomicBool>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            should_hide: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn set_should_hide(&self, value: bool) {
        self.should_hide.store(value, Ordering::SeqCst);
    }
    
    pub fn get_should_hide(&self) -> bool {
        self.should_hide.load(Ordering::SeqCst)
    }
}
```

**方案 2：Netlink Socket**
```rust
// 内核模块发送消息到用户态
// 用户态接收并处理
```

**方案 3：prctl 扩展**
```rust
// 使用 prctl 传递数据
const PR_WXSHADOW_QUERY: i32 = 0x57580009;

pub fn query_should_hide(caller: &str, target: &str) -> Result<bool> {
    // 将字符串编码为参数
    let result = unsafe {
        libc::prctl(PR_WXSHADOW_QUERY, caller.as_ptr(), target.as_ptr(), 0, 0)
    };
    Ok(result == 1)
}
```

## 性能优化

### 1. 缓存机制

```rust
use std::collections::HashMap;
use std::sync::Mutex;

pub struct HookCache {
    cache: Mutex<HashMap<(String, String), bool>>,
}

impl HookCache {
    pub fn get_or_compute(&self, caller: &str, target: &str, compute: impl FnOnce() -> bool) -> bool {
        let key = (caller.to_string(), target.to_string());
        
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(&result) = cache.get(&key) {
            return result;
        }
        
        let result = compute();
        cache.insert(key, result);
        result
    }
}
```

### 2. 批量处理

```rust
pub fn batch_check_apps(&self, caller: &str, targets: &[String]) -> Vec<bool> {
    targets.iter()
        .map(|target| self.should_hide(caller, target))
        .collect()
}
```

## 调试技巧

### 1. 日志记录

```rust
// 在 Hook 回调中记录日志
log::debug!("Hook triggered: caller={}, target={}", caller, target);
```

### 2. 性能分析

```rust
use std::time::Instant;

let start = Instant::now();
let result = self.should_hide(caller, target);
let duration = start.elapsed();

if duration.as_millis() > 10 {
    log::warn!("Slow hook: {}ms", duration.as_millis());
}
```

### 3. 内存检查

```rust
// 检查 Hook 是否被破坏
pub fn verify_hook(&self, addr: u64) -> Result<bool> {
    // 读取内存
    let mem = read_process_memory(self.pid, addr, 16)?;
    
    // 验证 Hook 代码
    Ok(mem == self.expected_hook_code)
}
```

## 安全考虑

### 1. 防止检测

- 使用 wxshadow 隐藏内存修改
- 随机化 Hook 地址
- 加密通信数据

### 2. 防止绕过

- 多层 Hook
- 监控 Hook 完整性
- 检测反 Hook 行为

### 3. 权限控制

- 限制 Hook 范围
- 验证调用者身份
- 审计 Hook 操作

## 未来改进

1. **完整的 rustFrida 集成**
   - 自动地址定位
   - 动态 Hook 管理
   - 脚本热更新

2. **更强的隐藏能力**
   - 多层混淆
   - 动态代码生成
   - 反检测对抗

3. **更好的性能**
   - 智能缓存
   - 异步处理
   - 批量优化

## 参考资料

- [Frida 官方文档](https://frida.re/docs/)
- [rustFrida 项目](https://github.com/example/rustfrida)
- [wxshadow 技术文档](../mkpms-master/CLAUDE.md)
