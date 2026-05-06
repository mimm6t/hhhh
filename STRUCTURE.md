# 项目结构

```
hide-my-applist-rust/
│
├── src/                            # 源代码目录
│   ├── lib.rs                      # 库入口 (34 行)
│   ├── wxshadow.rs                 # wxshadow FFI 封装 (119 行)
│   ├── config.rs                   # 配置管理 (177 行)
│   ├── process.rs                  # 进程和内存工具 (156 行)
│   ├── hook.rs                     # Hook 引擎 (205 行)
│   ├── advanced_hook.rs            # 高级 Hook 实现 (256 行)
│   └── bin/
│       └── main.rs                 # 命令行工具 (146 行)
│
├── Cargo.toml                      # Rust 项目配置
├── Cargo.lock                      # 依赖锁定文件
│
├── build.sh                        # 构建脚本 (98 行)
├── config.example.json             # 配置示例
│
├── README.md                       # 项目说明 (276 行)
├── QUICKSTART.md                   # 快速开始指南 (123 行)
├── ANALYSIS.md                     # 技术分析 (280 行)
├── DEPLOYMENT.md                   # 部署指南 (445 行)
├── RUSTFRIDA_INTEGRATION.md        # rustFrida 集成 (440 行)
├── SUMMARY.md                      # 项目总结 (495 行)
│
└── target/                         # 编译输出目录
    ├── debug/                      # 调试版本
    └── release/                    # 发布版本
        └── hma-rust                # 可执行文件
```

## 模块说明

### 核心模块

#### 1. wxshadow.rs
wxshadow 内核模块的 Rust FFI 封装

**功能：**
- prctl 系统调用封装
- ARM64 指令生成
- 错误处理

**主要函数：**
```rust
pub fn set_breakpoint(pid: i32, addr: u64) -> Result<()>
pub fn write_patch(pid: i32, addr: u64, data: &[u8]) -> Result<()>
pub fn release_shadow(pid: i32, addr: u64) -> Result<()>
```

#### 2. config.rs
配置管理系统

**功能：**
- JSON 配置解析
- 黑白名单管理
- 模板系统
- 应用过滤逻辑

**主要结构：**
```rust
pub struct Config {
    pub scope: HashMap<String, AppConfig>,
    pub templates: HashMap<String, Template>,
}
```

#### 3. process.rs
进程和内存操作工具

**功能：**
- `/proc/[pid]/maps` 解析
- 进程查找
- 内存映射分析
- 地址计算

**主要函数：**
```rust
pub fn parse_maps(pid: i32) -> Result<Vec<MemoryMap>>
pub fn find_process_by_name(name: &str) -> Result<Option<i32>>
pub fn find_library_executable_maps(pid: i32, lib_name: &str) -> Result<Vec<MemoryMap>>
```

#### 4. hook.rs
Hook 引擎核心

**功能：**
- system_server 定位
- Hook 安装管理
- 版本适配
- 应用过滤

**主要结构：**
```rust
pub struct PmsHookEngine {
    config: Arc<Mutex<Config>>,
    system_apps: HashSet<String>,
    system_server_pid: Option<i32>,
    hooks: Vec<HookTarget>,
}
```

#### 5. advanced_hook.rs
高级 Hook 实现

**功能：**
- Inline Hook 生成
- Trampoline 代码
- 符号解析
- 模式匹配

**主要结构：**
```rust
pub struct InlineHook {
    pub target_addr: u64,
    pub hook_addr: u64,
    pub original_bytes: Vec<u8>,
    pub trampoline: Vec<u32>,
}
```

#### 6. main.rs
命令行工具

**功能：**
- 配置管理
- Hook 安装/卸载
- 测试工具
- 状态监控

**主要命令：**
```bash
hma-rust install [config_path]  # 安装 Hook
hma-rust uninstall              # 卸载 Hook
hma-rust config <path>          # 管理配置
hma-rust test                   # 测试 wxshadow
hma-rust version                # 显示版本
```

### 文档

#### README.md
项目主文档，包含：
- 项目简介
- 技术架构
- 使用方法
- 配置说明
- 故障排除

#### QUICKSTART.md
快速开始指南，5 分钟上手

#### ANALYSIS.md
技术分析文档，包含：
- Hide-My-Applist 原版分析
- mkpms/wxshadow 技术分析
- 重写方案设计
- 实现难点

#### DEPLOYMENT.md
详细部署指南，包含：
- 前置要求
- 部署步骤
- 配置说明
- 故障排除
- 性能优化

#### RUSTFRIDA_INTEGRATION.md
rustFrida 集成文档，包含：
- 集成架构
- 实现方案
- 代码示例
- 性能优化

#### SUMMARY.md
项目总结，包含：
- 核心创新
- 技术亮点
- 性能指标
- 未来展望

### 配置文件

#### config.example.json
配置示例文件，包含：
- 完整配置结构
- 示例配置
- 注释说明

### 构建脚本

#### build.sh
自动化构建脚本，支持：
- 主机编译
- Android 交叉编译
- 自动配置
- 错误检查

## 依赖关系

```
hide-my-applist-rust
├── libc (0.2)              # C 库绑定
├── serde (1.0)             # 序列化/反序列化
├── serde_json (1.0)        # JSON 支持
├── anyhow (1.0)            # 错误处理
├── thiserror (1.0)         # 错误定义
├── log (0.4)               # 日志接口
├── env_logger (0.11)       # 日志实现
└── ctrlc (3.4)             # Ctrl+C 处理
```

## 编译产物

### Debug 版本
```
target/debug/
├── hma-rust                # 可执行文件 (~10 MB)
└── libhide_my_applist_rust.rlib  # 库文件
```

### Release 版本
```
target/release/
├── hma-rust                # 可执行文件 (~2 MB)
└── libhide_my_applist_rust.rlib  # 库文件
```

### Android 版本
```
target/aarch64-linux-android/release/
└── hma-rust                # Android 可执行文件 (~2 MB)
```

## 代码统计

| 类型 | 文件数 | 行数 |
|------|--------|------|
| Rust 源码 | 7 | ~1,093 |
| 文档 | 6 | ~2,059 |
| 配置 | 2 | ~50 |
| 脚本 | 1 | ~98 |
| **总计** | **16** | **~3,300** |

## 模块依赖图

```
main.rs
  ├─→ lib.rs
  │    ├─→ wxshadow.rs
  │    ├─→ config.rs
  │    ├─→ process.rs
  │    ├─→ hook.rs
  │    │    ├─→ config.rs
  │    │    ├─→ process.rs
  │    │    └─→ wxshadow.rs
  │    └─→ advanced_hook.rs
  │         ├─→ wxshadow.rs
  │         └─→ process.rs
  └─→ 外部依赖
       ├─→ libc
       ├─→ serde
       ├─→ anyhow
       └─→ ...
```

## 开发工作流

```
1. 编辑代码
   ↓
2. cargo check (快速检查)
   ↓
3. cargo test (运行测试)
   ↓
4. cargo build --release (编译发布版)
   ↓
5. ./build.sh (交叉编译 Android)
   ↓
6. adb push (部署到设备)
   ↓
7. 测试验证
```

## 目录权限

```
hide-my-applist-rust/
├── src/                    # 644 (rw-r--r--)
├── Cargo.toml              # 644 (rw-r--r--)
├── build.sh                # 755 (rwxr-xr-x)
├── *.md                    # 644 (rw-r--r--)
└── target/                 # 755 (rwxr-xr-x)
```

## 文件大小

| 文件 | 大小 |
|------|------|
| src/*.rs | ~50 KB |
| *.md | ~150 KB |
| Cargo.toml | ~1 KB |
| config.example.json | ~1 KB |
| target/release/hma-rust | ~2 MB |

## 版本控制

建议 `.gitignore` 内容：
```
/target/
Cargo.lock
*.swp
*.swo
*~
.DS_Store
```

## 备份建议

重要文件：
- `src/` - 源代码
- `*.md` - 文档
- `Cargo.toml` - 项目配置
- `config.example.json` - 配置示例
- `build.sh` - 构建脚本

可忽略：
- `target/` - 编译产物（可重新生成）
- `Cargo.lock` - 依赖锁定（可重新生成）
