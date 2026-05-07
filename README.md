# rustFrida KernelSU Module

将 [rustFrida](https://github.com/suifei/rustFrida) 打包为 KernelSU/Magisk/APatch 模块，开机自动启动 rustfrida server 模式。

## 特性

- ✅ 开机自动启动 rustfrida server
- ✅ HTTP RPC API (默认端口 27042)
- ✅ 支持属性伪装 (--profile)
- ✅ 支持 spawn 模式注入
- ✅ 支持 eBPF SO 监控
- ✅ 内置 REPL 和脚本执行
- ✅ 支持 KernelSU/Magisk/APatch

## 安装

1. 从 [Releases](../../releases) 下载 `rustFrida-KernelSU-x.x.x.zip`
2. 在 KernelSU/Magisk 管理器中安装
3. 重启设备

## 验证

```bash
# 检查进程
adb shell ps | grep rustfrida

# 测试 RPC
adb shell curl http://127.0.0.1:27042/sessions
```

## 配置

编辑 `/data/adb/modules/rustfrida-kernelsu/config/server.conf`:

```bash
RPC_PORT=27042
LISTEN_ADDR=0.0.0.0
# PROFILE=default
# VERBOSE=1
```

重启服务:
```bash
adb shell sh /data/adb/modules/rustfrida-kernelsu/action.sh
```

## 使用

### RPC API

```bash
# 列出 sessions
curl http://127.0.0.1:27042/sessions

# 调用 RPC 方法
curl -X POST http://127.0.0.1:27042/rpc/0/methodName \
  -H "Content-Type: application/json" \
  -d '[arg1, arg2]'
```

### 手动注入

```bash
# 进入 server REPL
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --server

# PID 注入
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --pid 1234

# Spawn 模式
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --spawn com.example.app
```

## 架构支持

目前仅支持 **arm64** (rustFrida 上游限制)

## 构建

```bash
# 克隆仓库
git clone https://github.com/mimm6t/hhhh.git
cd hhhh

# 构建 (需要 Android NDK 和 Rust)
python3 build.py
```

## 日志

```bash
# 查看启动日志
cat /data/adb/modules/rustfrida-kernelsu/logs/service.log

# 查看 rustfrida 输出
cat /data/adb/modules/rustfrida-kernelsu/logs/rustfrida.log
```

## 故障排查

### 服务未启动

```bash
# 手动启动测试
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --server --verbose
```

### RPC 无法连接

```bash
# 检查端口
netstat -tlnp | grep 27042

# 测试本地连接
curl -v http://127.0.0.1:27042/sessions
```

## 限制

- ❌ 仅支持 arm64 架构
- ❌ 不兼容官方 frida-tools
- ❌ 二进制较大 (~15MB)

## 许可

基于 rustFrida 项目，遵循其原始许可证。

## 参考

- [rustFrida](https://github.com/suifei/rustFrida)
- [magisk-frida](https://github.com/ViRb3/magisk-frida)
