# Hide-My-Applist Rust - 快速参考

## 一键命令

### 编译
```bash
cd hide-my-applist-rust && ./build.sh
```

### 部署
```bash
adb push target/aarch64-linux-android/release/{hma-rust,symbol-test} /data/local/tmp/
adb shell chmod +x /data/local/tmp/{hma-rust,symbol-test}
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"
```

### 测试
```bash
adb shell su -c "/data/local/tmp/symbol-test version"
adb shell su -c "/data/local/tmp/hma-rust test"
```

### 运行
```bash
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/config.json"
```

## 核心模块

| 模块 | 功能 | 行数 |
|------|------|------|
| `elf.rs` | ELF 解析 | 127 |
| `symbol.rs` | 符号解析 | 77 |
| `android.rs` | 版本适配 | 104 |
| `pms_hook.rs` | PMS Hook | 204 |

## 支持的 Android 版本

| 版本 | API | 状态 |
|------|-----|------|
| Android 9 | 28 | ⚠️ 需要 Java Hook |
| Android 10 | 29 | ⚠️ 需要 Java Hook |
| Android 11 | 30 | ✅ 完全支持 |
| Android 12 | 31 | ✅ 完全支持 |
| Android 12L | 32 | ✅ 完全支持 |
| Android 13 | 33 | ✅ 完全支持 |
| Android 14 | 34 | ✅ 完全支持 |
| Android 15+ | 35+ | ✅ 完全支持 |

## 常用命令

### symbol-test
```bash
# 检查版本
symbol-test version

# 解析 ELF
symbol-test parse /system/lib64/libc.so

# 解析符号
symbol-test resolve <pid> libc.so malloc

# 查看内存映射
symbol-test maps <pid>
```

### hma-rust
```bash
# 测试 wxshadow
hma-rust test

# 验证配置
hma-rust config <path>

# 安装 Hook
hma-rust install <config>

# 卸载 Hook
hma-rust uninstall
```

## 配置模板

### 黑名单模式
```json
{
  "scope": {
    "com.example.app": {
      "use_whitelist": false,
      "extra_app_list": ["com.topjohnwu.magisk"]
    }
  }
}
```

### 白名单模式
```json
{
  "scope": {
    "com.example.app": {
      "use_whitelist": true,
      "extra_app_list": ["com.android.chrome"]
    }
  }
}
```

## 故障排除

### wxshadow 不可用
```bash
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"
adb shell su -c "dmesg | grep wxshadow"
```

### 符号解析失败
```bash
adb shell readelf -s /system/lib64/libandroid_servers.so | grep shouldFilter
```

### Hook 不生效
```bash
adb shell su -c "dmesg | grep wxshadow | tail -20"
adb shell su -c "setenforce 0"
```

## 文档索引

| 文档 | 用途 |
|------|------|
| `README.md` | 项目说明 |
| `QUICKSTART.md` | 5 分钟上手 |
| `IMPLEMENTATION.md` | 实现细节 |
| `TESTING.md` | 测试指南 |
| `DEPLOYMENT.md` | 部署指南 |

## 性能指标

| 指标 | 目标 |
|------|------|
| CPU 使用率 | < 1% |
| 内存占用 | < 10 MB |
| 启动延迟 | < 100ms |
| Hook 延迟 | < 1ms |
| 二进制大小 | ~2 MB |

## 技术栈

- **语言：** Rust 2024
- **内核模块：** wxshadow (W^X Shadow)
- **框架：** KernelPatch / APatch
- **架构：** ARM64

## 核心优势

✅ 内核级别隐藏  
✅ W^X Shadow 技术  
✅ 自动符号解析  
✅ 多版本适配  
✅ 内存安全  
✅ 高性能  

## 项目状态

**版本：** v0.2.0  
**状态：** ✅ 核心功能完成  
**完成度：** 85%  
**下一步：** 设备测试  

## 获取帮助

- **文档：** 查看 `docs/` 目录
- **问题：** 提交 GitHub Issue
- **讨论：** 加入讨论区

---

**最后更新：** 2026-05-06
