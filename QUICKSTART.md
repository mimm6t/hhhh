# 快速开始指南

5 分钟快速上手 Hide-My-Applist Rust

## 前置条件

- ✅ Android 设备已 Root
- ✅ 已安装 KernelPatch 或 APatch
- ✅ 电脑已安装 ADB

## 步骤 1：获取 wxshadow.kpm

从 mkpms 项目获取或下载预编译的 wxshadow.kpm

## 步骤 2：加载内核模块

```bash
adb push wxshadow.kpm /data/local/tmp/
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"
```

验证：
```bash
adb shell su -c "dmesg | grep wxshadow"
```

应该看到：`[wxshadow] Module initialized`

## 步骤 3：编译程序

```bash
cd hide-my-applist-rust
./build.sh
```

## 步骤 4：部署到设备

```bash
adb push target/aarch64-linux-android/release/hma-rust /data/local/tmp/
adb shell chmod +x /data/local/tmp/hma-rust
```

## 步骤 5：测试

```bash
adb shell su -c "/data/local/tmp/hma-rust test"
```

应该看到：`✓ wxshadow is available`

## 步骤 6：配置

创建 `config.json`：

```json
{
  "config_version": 1,
  "detail_log": true,
  "max_log_size": 1024,
  "scope": {
    "com.example.targetapp": {
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
```

推送到设备：
```bash
adb push config.json /data/local/tmp/hma_config.json
```

## 步骤 7：运行

```bash
adb shell su -c "/data/local/tmp/hma-rust install /data/local/tmp/hma_config.json"
```

## 验证效果

1. 打开目标应用
2. 查看已安装应用列表
3. 确认配置的应用已被隐藏

## 故障排除

### wxshadow 不可用

```bash
# 检查模块
adb shell su -c "kpatch module list"

# 重新加载
adb shell su -c "kpatch module load /data/local/tmp/wxshadow.kpm"
```

### 找不到 system_server

```bash
# 检查进程
adb shell ps -A | grep system_server

# 重启设备
adb reboot
```

## 下一步

- 阅读 [README.md](README.md) 了解详细功能
- 阅读 [DEPLOYMENT.md](DEPLOYMENT.md) 了解部署细节
- 阅读 [ANALYSIS.md](ANALYSIS.md) 了解技术原理

## 获取帮助

- 查看日志：`adb shell su -c "dmesg | grep wxshadow"`
- 提交 Issue：GitHub Issues
- 技术交流：讨论区
