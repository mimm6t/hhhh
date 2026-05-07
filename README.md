# rustFrida KernelSU Module

将 [rustFrida](https://github.com/suifei/rustFrida) 打包为 KernelSU/Magisk 模块，开机自启 server 模式。

## 特性

✅ 开机自启 rustfrida server  
✅ HTTP RPC API (端口 27042)  
✅ 支持属性伪装 (--profile)  
✅ 支持 spawn/watch-so 注入  
✅ 支持 KernelSU/Magisk/APatch  

## 安装

1. 从 [Releases](../../releases) 下载 ZIP
2. 在 KernelSU/Magisk 管理器安装
3. 重启设备

## 验证

```bash
adb shell ps | grep rustfrida
adb shell curl http://127.0.0.1:27042/sessions
```

## 配置

编辑 `/data/adb/modules/rustfrida-kernelsu/config/server.conf`:

```bash
RPC_PORT=27042
LISTEN_ADDR=0.0.0.0
```

重启: `adb shell sh /data/adb/modules/rustfrida-kernelsu/action.sh`

## 使用

### RPC API

```bash
curl http://127.0.0.1:27042/sessions
curl -X POST http://127.0.0.1:27042/rpc/0/method -H "Content-Type: application/json" -d '[args]'
```

### 手动注入

```bash
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --pid 1234
/data/adb/modules/rustfrida-kernelsu/bin/rustfrida --spawn com.app
```

## 限制

- 仅支持 arm64
- 不兼容 frida-tools

## 参考

- [rustFrida](https://github.com/suifei/rustFrida)
- [magisk-frida](https://github.com/ViRb3/magisk-frida)
