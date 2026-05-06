# Hide-My-Applist Rust

![Build](https://github.com/mimm6t/hhhh/workflows/Build/badge.svg)
![Check](https://github.com/mimm6t/hhhh/workflows/Check/badge.svg)
![Release](https://github.com/mimm6t/hhhh/workflows/Release/badge.svg)

基于 wxshadow 内核模块的应用列表隐藏工具 - Hide-My-Applist 的 Rust 重写版本

## 项目简介

这是 [Hide-My-Applist](https://github.com/Dr-TSNG/Hide-My-Applist) 的重写版本，使用 **wxshadow 内核模块** 和 **Rust** 实现，相比原版 Xposed 方案具有以下优势：

- ✅ **内核级别隐藏**：Hook 在内核层，用户态无法检测
- ✅ **无痕 Hook**：利用 W^X Shadow 技术，内存校验无法发现修改
- ✅ **更高性能**：Rust 实现，零成本抽象
- ✅ **更强稳定性**：不依赖 Xposed 框架

**当前状态：**
- ✅ 核心功能完成（Rust 库 + 命令行工具）
- ⏳ Android UI 开发中（详见 [ANDROID_UI_PLAN.md](ANDROID_UI_PLAN.md)）

**临时使用方式：** 命令行工具 + 配置文件

## 技术架构

```
┌─────────────────────────────────────────┐
│     Hide-My-Applist Rust                │
│  ┌───────────────────────────────────┐  │
│  │  配置管理 (config.rs)              │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Hook 引擎 (hook.rs)               │  │
│  │  - PMS Hook                        │  │
│  │  - Binder Hook                     │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  wxshadow 接口 (wxshadow.rs)       │  │
│  │  - prctl 封装                      │  │
│  │  - ARM64 指令生成                  │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
              ↓ prctl syscall
┌─────────────────────────────────────────┐
│      wxshadow.kpm (内核模块)            │
│  - W^X Shadow Memory                    │
│  - 断点处理                             │
│  - 页面切换                             │
└─────────────────────────────────────────┘
```

## 核心技术：W^X Shadow

wxshadow 通过内核级别的页面切换实现无痕 Hook：

1. **Shadow Page**：为目标代码页创建副本，写入 Hook 代码
2. **读写分离**：
   - 读取时：返回原始页内容（`r--` 权限）
   - 执行时：使用 Shadow 页（`--x` 权限）
3. **无痕效果**：内存校验读取到的是原始内容，无法发现 Hook

## 依赖要求

### 必需
- **KernelPatch** 或 **APatch**：用于加载内核模块
- **wxshadow.kpm**：W^X Shadow 内核模块
- **Root 权限**：需要操作内核和系统进程

### 可选
- **rustFrida**：用于更高级的 Hook 功能

## 编译

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
cd /path/to/hide-my-applist-rust

# 编译
cargo build --release

# 编译 Android 版本（需要 NDK）
cargo build --release --target aarch64-linux-android
```

## 使用方法

### 1. 准备工作

```bash
# 加载 wxshadow 内核模块
adb push wxshadow.kpm /data/local/tmp/
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"

# 推送程序到设备
adb push target/aarch64-linux-android/release/hma-rust /data/local/tmp/
adb shell chmod +x /data/local/tmp/hma-rust
```

### 2. 配置

创建配置文件 `config.json`：

```json
{
  "config_version": 1,
  "detail_log": true,
  "max_log_size": 1024,
  "scope": {
    "com.example.detector": {
      "use_whitelist": false,
      "exclude_system_apps": true,
      "extra_app_list": [
        "com.topjohnwu.magisk",
        "de.robv.android.xposed.installer"
      ],
      "apply_templates": ["root_tools"]
    }
  },
  "templates": {
    "root_tools": {
      "name": "Root Tools",
      "app_list": [
        "com.topjohnwu.magisk",
        "me.weishu.kernelsu",
        "me.bmax.apatch"
      ]
    }
  }
}
```

### 3. 运行

```bash
# 测试 wxshadow 是否可用
adb shell su -c "/data/local/tmp/hma-rust test"

# 安装 Hook
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/config.json"

# 卸载 Hook
adb shell su -c "/data/local/tmp/hma-rust uninstall"
```

## 配置说明

### 全局配置

- `config_version`: 配置版本号
- `detail_log`: 是否启用详细日志
- `max_log_size`: 最大日志大小（KB）

### Scope 配置

为每个需要隐藏应用列表的应用配置规则：

```json
"com.example.app": {
  "use_whitelist": false,        // false=黑名单模式，true=白名单模式
  "exclude_system_apps": true,   // 是否排除系统应用
  "extra_app_list": [...],       // 额外的应用列表
  "apply_templates": [...]       // 应用的模板
}
```

### 模板

定义可复用的应用列表：

```json
"root_tools": {
  "name": "Root Tools",
  "app_list": [
    "com.topjohnwu.magisk",
    "me.weishu.kernelsu"
  ]
}
```

## 工作原理

### Hook 流程

1. **定位目标**：找到 `system_server` 进程
2. **解析内存**：读取 `/proc/[pid]/maps` 找到目标库
3. **设置 Hook**：使用 wxshadow 在关键函数设置断点或 patch
4. **拦截调用**：当应用查询应用列表时触发 Hook
5. **过滤结果**：根据配置过滤返回的应用列表

### Hook 目标

根据 Android 版本选择不同的 Hook 点：

- **Android 14+**: `AppsFilterImpl.shouldFilterApplication()`
- **Android 13**: `Computer.getPackagesForUid()`
- **Android 11-12**: `PackageManagerService` 相关方法
- **Android 9-10**: `getInstalledPackages()` 等

## 开发状态

### 已完成
- [x] wxshadow FFI 封装
- [x] 配置管理模块
- [x] 进程和内存工具
- [x] Hook 引擎框架
- [x] 命令行工具

### 进行中
- [ ] rustFrida 集成
- [ ] PMS 方法地址定位
- [ ] 实际 Hook 实现
- [ ] 多版本适配

### 计划中
- [ ] GUI 配置工具
- [ ] 自动更新机制
- [ ] 性能优化
- [ ] 完整测试

## 与原版对比

| 特性 | 原版 (Xposed) | Rust 版 (wxshadow) |
|------|---------------|-------------------|
| Hook 层级 | 用户态 Java | 内核态 |
| 检测难度 | 容易 | 极难 |
| 内存扫描 | 可检测 | 不可检测 |
| 依赖框架 | Xposed/LSPosed | KernelPatch |
| 性能 | 中等 | 高 |
| 稳定性 | 依赖框架 | 独立稳定 |

## 注意事项

1. **需要 Root**：必须有 root 权限才能使用
2. **内核模块**：必须先加载 wxshadow.kpm
3. **兼容性**：目前仅支持 ARM64 架构
4. **实验性**：项目仍在开发中，可能不稳定

## 故障排除

### wxshadow 不可用

```bash
# 检查内核模块是否加载
adb shell su -c "kpatch module list"

# 查看内核日志
adb shell su -c "dmesg | grep wxshadow"
```

### Hook 失败

```bash
# 检查 system_server 进程
adb shell ps -A | grep system_server

# 查看详细日志
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/config.json"
```

## 参考资料

- [Hide-My-Applist 原版](https://github.com/Dr-TSNG/Hide-My-Applist)
- [mkpms (wxshadow)](https://github.com/example/mkpms)
- [KernelPatch](https://github.com/bmax121/KernelPatch)
- [APatch](https://github.com/bmax121/APatch)

## 更新日志

### v0.2.0 (2026-05-06)

**新增功能：**
- ✅ 完整的 ELF 解析器 (`elf.rs`)
- ✅ 符号解析系统 (`symbol.rs`)
- ✅ Android 9-15 版本适配 (`android.rs`)
- ✅ 实际 PMS Hook 实现 (`pms_hook.rs`)
- ✅ 符号测试工具 (`symbol-test`)

**改进：**
- 自动检测 Android 版本
- 自动解析符号地址
- 支持多版本 Hook 策略
- 符号表缓存优化

**已知问题：**
- Android 9-10 需要 Java Hook (待实现)
- 某些 ROM 可能需要手动适配

详见 [IMPLEMENTATION.md](IMPLEMENTATION.md)

## 许可证

GPL-3.0 License

## 免责声明

本项目仅供学习和研究使用，请勿用于非法用途。使用本项目产生的一切后果由使用者自行承担。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- Hide-My-Applist 原作者
- wxshadow/mkpms 作者
- KernelPatch 团队
