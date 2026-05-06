# 完整功能实现说明

## 新增功能

### 1. ELF 解析 (`elf.rs`)

完整的 ELF64 解析器，支持：
- 解析 ELF 头
- 读取节头表
- 提取符号表 (SYMTAB 和 DYNSYM)
- 解析字符串表
- 符号查找

**使用示例：**
```rust
use hide_my_applist_rust::elf;

let symbols = elf::parse_symbols("/system/lib64/libc.so")?;
if let Some(sym) = elf::find_symbol(&symbols, "malloc") {
    println!("malloc at 0x{:x}", sym.addr);
}
```

### 2. 符号解析 (`symbol.rs`)

自动处理 ASLR 的符号解析器：
- 解析进程内存映射
- 计算库基址
- 缓存符号表
- 批量符号解析

**使用示例：**
```rust
use hide_my_applist_rust::symbol::SymbolResolver;

let mut resolver = SymbolResolver::new();
let addr = resolver.resolve(pid, "libc.so", "malloc")?;
```

### 3. Android 版本适配 (`android.rs`)

支持 Android 9-15 (API 28-35+)：
- 自动检测 Android 版本
- 版本特定的 Hook 目标
- 框架路径适配

**支持的版本：**
- Android 9 (API 28)
- Android 10 (API 29)
- Android 11 (API 30)
- Android 12 (API 31)
- Android 12L (API 32)
- Android 13 (API 33)
- Android 14 (API 34)
- Android 15+ (API 35+)

**使用示例：**
```rust
use hide_my_applist_rust::android::AndroidVersion;

let version = AndroidVersion::detect()?;
println!("Running on Android {}", version.sdk_int());
```

### 4. 实际 PMS Hook (`pms_hook.rs`)

完整的 PMS Hook 实现：
- 自动版本检测
- 符号解析
- Hook 安装
- 多版本适配

**Hook 策略：**

#### Android 14-15
- Hook: `shouldFilterApplication` in `libandroid_servers.so`
- 方法：直接返回 false (不过滤)

#### Android 13
- Hook: `shouldFilterApplication` in `libandroid_servers.so`
- 方法：直接返回 false

#### Android 11-12
- Hook: `shouldFilterApplication` in framework
- 方法：直接返回 false

#### Android 9-10
- Hook: `getInstalledPackages` / `getInstalledApplications`
- 方法：需要 Java Hook (待实现)

**使用示例：**
```rust
use hide_my_applist_rust::pms_hook::PmsHook;

let mut hook = PmsHook::new(pid, config)?;
hook.install()?;
```

### 5. 符号测试工具 (`symbol-test`)

新增命令行工具用于测试和调试：

```bash
# 解析 ELF 符号
symbol-test parse /system/lib64/libc.so

# 解析进程中的符号
symbol-test resolve <pid> libc.so malloc

# 检查 Android 版本
symbol-test version

# 查看进程内存映射
symbol-test maps <pid>
```

## 完整使用流程

### 1. 检查环境

```bash
# 检查 wxshadow 是否加载
adb shell su -c "dmesg | grep wxshadow"

# 检查 Android 版本
adb shell su -c "/data/local/tmp/symbol-test version"
```

### 2. 测试符号解析

```bash
# 查找 system_server 进程
adb shell ps -A | grep system_server

# 查看内存映射
adb shell su -c "/data/local/tmp/symbol-test maps <pid>"

# 解析符号
adb shell su -c "/data/local/tmp/symbol-test resolve <pid> libandroid_servers.so shouldFilterApplication"
```

### 3. 安装 Hook

```bash
# 使用配置文件
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/config.json"
```

### 4. 验证效果

打开目标应用，检查应用列表是否被正确隐藏。

## 技术细节

### ELF 解析流程

```
1. 读取 ELF 头 (64 字节)
   ↓
2. 解析节头表偏移和数量
   ↓
3. 读取所有节头
   ↓
4. 查找符号表节 (SHT_SYMTAB/SHT_DYNSYM)
   ↓
5. 读取对应的字符串表
   ↓
6. 解析符号条目
   ↓
7. 返回符号列表
```

### 符号解析流程

```
1. 解析 /proc/[pid]/maps
   ↓
2. 查找目标库的基址
   ↓
3. 解析库的 ELF 符号表
   ↓
4. 查找目标符号
   ↓
5. 计算实际地址 = 基址 + 符号偏移
   ↓
6. 返回绝对地址
```

### Hook 安装流程

```
1. 检测 Android 版本
   ↓
2. 选择对应的 Hook 策略
   ↓
3. 解析目标符号地址
   ↓
4. 生成 Hook 代码
   ↓
5. 使用 wxshadow 写入 patch
   ↓
6. 验证 Hook 安装
```

## 性能优化

### 1. 符号表缓存

符号表解析较慢，使用缓存避免重复解析：

```rust
pub struct SymbolResolver {
    cache: HashMap<String, Vec<elf::Symbol>>,
}
```

### 2. 批量解析

一次解析多个符号，减少 I/O：

```rust
let addrs = resolver.resolve_multiple(pid, "libc.so", &[
    "malloc", "free", "calloc"
])?;
```

### 3. 延迟加载

只在需要时才解析符号表。

## 已知限制

### 1. Android 9-10 支持

Android 9-10 的 PMS 方法是 Java 实现，需要 Java Hook：
- 当前实现：仅记录警告
- 计划：集成 rustFrida 实现 Java Hook

### 2. 符号混淆

某些 ROM 可能对系统库进行符号混淆：
- 解决方案：使用模式匹配查找
- 备选方案：手动指定地址

### 3. 内存读取权限

读取 `/proc/[pid]/mem` 需要特定权限：
- 需要 root 权限
- SELinux 可能阻止访问

## 调试技巧

### 1. 查看符号表

```bash
# 使用 readelf
adb shell readelf -s /system/lib64/libandroid_servers.so | grep shouldFilter

# 使用 symbol-test
adb shell su -c "/data/local/tmp/symbol-test parse /system/lib64/libandroid_servers.so"
```

### 2. 验证地址

```bash
# 查看内存映射
adb shell su -c "cat /proc/<pid>/maps | grep libandroid_servers"

# 计算实际地址
# 实际地址 = 基址 + 符号偏移
```

### 3. 测试 Hook

```bash
# 查看内核日志
adb shell su -c "dmesg | grep wxshadow"

# 查看应用日志
adb logcat | grep HMA
```

## 下一步计划

### 短期 (1-2 周)

1. ✅ ELF 解析
2. ✅ 符号解析
3. ✅ Android 版本适配
4. ✅ 实际 PMS Hook
5. ⏳ 完整测试

### 中期 (1 个月)

1. rustFrida 集成
2. Java Hook 支持
3. 动态配置更新
4. 性能优化

### 长期 (2-3 个月)

1. GUI 配置工具
2. 自动更新
3. 更多 ROM 适配
4. 完整文档

## 贡献指南

欢迎贡献代码！重点领域：

1. **更多 Android 版本测试**
   - 测试不同 ROM
   - 报告兼容性问题

2. **符号解析优化**
   - 支持更多符号格式
   - 提高解析速度

3. **Hook 策略改进**
   - 更智能的过滤逻辑
   - 更好的性能

4. **文档完善**
   - 添加更多示例
   - 翻译文档

## 参考资料

- [ELF Format Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [Android Source Code](https://cs.android.com/)
- [wxshadow Documentation](../mkpms-master/CLAUDE.md)
