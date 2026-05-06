# 测试和验证指南

本文档提供完整的测试流程，验证所有新功能是否正常工作。

## 环境准备

### 1. 编译所有工具

```bash
cd hide-my-applist-rust
./build.sh
```

应该生成：
- `target/aarch64-linux-android/release/hma-rust`
- `target/aarch64-linux-android/release/symbol-test`

### 2. 推送到设备

```bash
adb push target/aarch64-linux-android/release/hma-rust /data/local/tmp/
adb push target/aarch64-linux-android/release/symbol-test /data/local/tmp/
adb shell chmod +x /data/local/tmp/hma-rust
adb shell chmod +x /data/local/tmp/symbol-test
```

### 3. 加载 wxshadow

```bash
adb push ../mkpms-master/build/kpms/wxshadow/wxshadow.kpm /data/local/tmp/
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"
```

验证：
```bash
adb shell su -c "dmesg | grep wxshadow"
```

## 测试 1: Android 版本检测

```bash
adb shell su -c "/data/local/tmp/symbol-test version"
```

**预期输出：**
```
Android Version: Android14
SDK Int: 34

Hook Targets:
  - shouldFilterApplication: _ZN7android6server2pm14AppsFilterImpl24shouldFilterApplicationEP
  - getPackagesForUid: getPackagesForUid

Target Library: libandroid_servers.so
Framework Path: /system/lib64/libandroid_servers.so
```

**验证点：**
- ✅ 正确检测到 Android 版本
- ✅ 显示对应的 Hook 目标
- ✅ 显示正确的库路径

## 测试 2: ELF 符号解析

### 2.1 解析系统库

```bash
adb shell su -c "/data/local/tmp/symbol-test parse /system/lib64/libc.so"
```

**预期输出：**
```
Parsing ELF: /system/lib64/libc.so
Found 2847 symbols
   1. 0x0000000000012340     128 malloc
   2. 0x0000000000012450      64 free
   ...
```

**验证点：**
- ✅ 成功解析 ELF 文件
- ✅ 找到符号表
- ✅ 显示符号名称和地址

### 2.2 解析 PMS 库

```bash
adb shell su -c "/data/local/tmp/symbol-test parse /system/lib64/libandroid_servers.so"
```

**预期输出：**
```
Parsing ELF: /system/lib64/libandroid_servers.so
Found 1523 symbols
   1. 0x0000000000045678    256 shouldFilterApplication
   ...
```

**验证点：**
- ✅ 找到 `shouldFilterApplication` 符号
- ✅ 地址非零

## 测试 3: 进程内存映射

### 3.1 查找 system_server

```bash
adb shell ps -A | grep system_server
```

记录 PID，例如：`1234`

### 3.2 查看内存映射

```bash
adb shell su -c "/data/local/tmp/symbol-test maps 1234"
```

**预期输出：**
```
Memory maps for process 1234

Executable mappings: 156
  0x00007b5c001000-0x00007b5c123000 r-xp /system/lib64/libc.so
  0x00007b5d001000-0x00007b5d456000 r-xp /system/lib64/libandroid_servers.so
  ...
```

**验证点：**
- ✅ 找到可执行映射
- ✅ 包含 `libandroid_servers.so`
- ✅ 地址范围合理

## 测试 4: 符号解析

```bash
adb shell su -c "/data/local/tmp/symbol-test resolve 1234 libandroid_servers.so shouldFilterApplication"
```

**预期输出：**
```
Resolving symbol in process 1234
Library: libandroid_servers.so
Symbol: shouldFilterApplication
✓ Found at: 0x7b5d045678
```

**验证点：**
- ✅ 成功解析符号
- ✅ 返回绝对地址
- ✅ 地址在库的映射范围内

## 测试 5: wxshadow 可用性

```bash
adb shell su -c "/data/local/tmp/hma-rust test"
```

**预期输出：**
```
✓ wxshadow is available
```

**如果失败：**
```
✗ wxshadow is NOT available: ...
Make sure wxshadow.kpm is loaded:
  kpatch module load /path/to/wxshadow.kpm
```

**验证点：**
- ✅ wxshadow 模块已加载
- ✅ prctl 接口可用

## 测试 6: 配置管理

### 6.1 创建测试配置

```bash
cat > /tmp/test_config.json << 'EOF'
{
  "config_version": 1,
  "detail_log": true,
  "max_log_size": 1024,
  "scope": {
    "com.android.settings": {
      "use_whitelist": false,
      "exclude_system_apps": true,
      "extra_app_list": [
        "com.topjohnwu.magisk"
      ],
      "apply_templates": []
    }
  },
  "templates": {}
}
EOF

adb push /tmp/test_config.json /data/local/tmp/test_config.json
```

### 6.2 验证配置

```bash
adb shell su -c "/data/local/tmp/hma-rust config /data/local/tmp/test_config.json"
```

**预期输出：**
```
Loading config from "/data/local/tmp/test_config.json"
Config loaded successfully
  Version: 1
  Scope entries: 1
  Templates: 0
```

**验证点：**
- ✅ 配置文件格式正确
- ✅ 成功解析 JSON
- ✅ 配置项正确

## 测试 7: Hook 安装

### 7.1 安装 Hook

```bash
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/test_config.json"
```

**预期输出：**
```
Loading configuration from "/data/local/tmp/test_config.json"
Initializing hook engine...
Found system_server PID: 1234
Detected Android version: Android14 (SDK 34)
Found shouldFilterApplication at 0x7b5d045678
Installing hooks for PID 1234
Installed 1 hooks
Hooks installed successfully!
Press Ctrl+C to uninstall and exit
```

**验证点：**
- ✅ 找到 system_server
- ✅ 检测到 Android 版本
- ✅ 解析到符号地址
- ✅ 成功安装 Hook

### 7.2 验证 Hook

在另一个终端：

```bash
# 查看内核日志
adb shell su -c "dmesg | grep wxshadow | tail -20"
```

**预期输出：**
```
[wxshadow] PATCH: pid=1234 addr=0x7b5d045678 len=8
[wxshadow] Shadow page allocated for 0x7b5d045000
```

**验证点：**
- ✅ wxshadow 记录了 patch 操作
- ✅ Shadow 页面已分配

## 测试 8: 功能验证

### 8.1 测试应用列表隐藏

1. 打开"设置"应用
2. 进入"应用管理"
3. 查看已安装应用列表
4. 确认 Magisk 是否被隐藏

**预期结果：**
- ✅ Magisk 不在列表中
- ✅ 其他应用正常显示

### 8.2 测试白名单模式

修改配置：
```json
{
  "scope": {
    "com.android.settings": {
      "use_whitelist": true,
      "extra_app_list": [
        "com.android.chrome"
      ]
    }
  }
}
```

重新安装 Hook，验证只显示 Chrome。

## 测试 9: 性能测试

### 9.1 CPU 使用率

```bash
# 安装 Hook 后
adb shell top -n 1 | grep hma-rust
```

**预期：**
- CPU 使用率 < 1%

### 9.2 内存占用

```bash
adb shell ps -A | grep hma-rust
```

**预期：**
- 内存占用 < 10 MB

### 9.3 响应延迟

打开应用列表，观察加载速度。

**预期：**
- 无明显延迟
- 与未安装 Hook 时相同

## 测试 10: 稳定性测试

### 10.1 长时间运行

让 Hook 运行 1 小时，期间：
- 多次打开/关闭应用列表
- 安装/卸载应用
- 重启应用

**验证点：**
- ✅ 无崩溃
- ✅ Hook 持续有效
- ✅ 无内存泄漏

### 10.2 压力测试

快速连续打开应用列表 100 次。

**验证点：**
- ✅ 无崩溃
- ✅ 响应正常
- ✅ 过滤正确

## 测试 11: 错误处理

### 11.1 无效配置

```bash
echo "invalid json" > /tmp/invalid.json
adb push /tmp/invalid.json /data/local/tmp/
adb shell su -c "/data/local/tmp/hma-rust config /data/local/tmp/invalid.json"
```

**预期：**
- ✅ 显示错误信息
- ✅ 不崩溃

### 11.2 wxshadow 未加载

```bash
# 卸载 wxshadow
adb shell su -c "kpatch module unload wxshadow"

# 尝试安装 Hook
adb shell su -c "/data/local/tmp/hma-rust test"
```

**预期：**
- ✅ 显示 wxshadow 不可用
- ✅ 提示加载模块

### 11.3 进程不存在

```bash
adb shell su -c "/data/local/tmp/symbol-test maps 99999"
```

**预期：**
- ✅ 显示错误信息
- ✅ 不崩溃

## 测试结果记录

| 测试项 | 状态 | 备注 |
|--------|------|------|
| Android 版本检测 | ⬜ | |
| ELF 符号解析 | ⬜ | |
| 进程内存映射 | ⬜ | |
| 符号解析 | ⬜ | |
| wxshadow 可用性 | ⬜ | |
| 配置管理 | ⬜ | |
| Hook 安装 | ⬜ | |
| 功能验证 | ⬜ | |
| 性能测试 | ⬜ | |
| 稳定性测试 | ⬜ | |
| 错误处理 | ⬜ | |

## 常见问题

### Q1: 符号解析失败

**可能原因：**
- 库路径错误
- 符号被混淆
- 权限不足

**解决方案：**
```bash
# 检查库是否存在
adb shell ls -l /system/lib64/libandroid_servers.so

# 使用 readelf 验证
adb shell readelf -s /system/lib64/libandroid_servers.so | grep shouldFilter
```

### Q2: Hook 安装失败

**可能原因：**
- wxshadow 未加载
- 地址错误
- SELinux 阻止

**解决方案：**
```bash
# 检查 wxshadow
adb shell su -c "dmesg | grep wxshadow"

# 临时关闭 SELinux
adb shell su -c "setenforce 0"
```

### Q3: 应用仍然可见

**可能原因：**
- Hook 未生效
- 配置错误
- 应用使用其他检测方法

**解决方案：**
```bash
# 检查日志
adb shell su -c "dmesg | grep wxshadow"

# 验证配置
adb shell su -c "/data/local/tmp/hma-rust config /data/local/tmp/config.json"
```

## 报告问题

如果测试失败，请提供：

1. 设备信息
   ```bash
   adb shell getprop ro.build.version.release
   adb shell getprop ro.build.version.sdk
   adb shell getprop ro.product.model
   ```

2. 完整日志
   ```bash
   adb shell su -c "dmesg > /data/local/tmp/dmesg.log"
   adb pull /data/local/tmp/dmesg.log
   ```

3. 测试输出
   - 复制所有命令和输出
   - 截图（如果适用）

4. 配置文件
   - 提供使用的配置文件

## 下一步

测试通过后：
1. 阅读 [IMPLEMENTATION.md](IMPLEMENTATION.md) 了解实现细节
2. 查看 [DEPLOYMENT.md](DEPLOYMENT.md) 了解部署方案
3. 参考 [RUSTFRIDA_INTEGRATION.md](RUSTFRIDA_INTEGRATION.md) 了解高级功能
