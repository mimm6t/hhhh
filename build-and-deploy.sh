#!/bin/bash
# 本地编译 Rust Web UI 并推送到设备

set -e

echo "=== 本地编译 rustfrida-webui ==="

# 检查 NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "错误: 请设置 ANDROID_NDK_HOME 环境变量"
    echo "例如: export ANDROID_NDK_HOME=~/Android/Sdk/ndk/25.0.8775105"
    exit 1
fi

# 检查 Rust target
if ! rustup target list | grep -q "aarch64-linux-android (installed)"; then
    echo "安装 aarch64-linux-android target..."
    rustup target add aarch64-linux-android
fi

# 设置环境变量
export CC_aarch64_linux_android="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"
export AR_aarch64_linux_android="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$CC_aarch64_linux_android"

# 编译
cd webui
echo "编译中..."
cargo build --release --target aarch64-linux-android

# 检查输出
BINARY="target/aarch64-linux-android/release/rustfrida-webui"
if [ ! -f "$BINARY" ]; then
    echo "错误: 编译失败"
    exit 1
fi

echo "编译成功: $BINARY"
ls -lh "$BINARY"

# 推送到设备
echo ""
echo "=== 推送到设备 ==="
adb push "$BINARY" /data/local/tmp/
adb shell "su -c 'cp /data/local/tmp/rustfrida-webui /data/adb/modules/rustfrida-kernelsu/bin/'"
adb shell "su -c 'chmod 755 /data/adb/modules/rustfrida-kernelsu/bin/rustfrida-webui'"

# 启动 Web UI
echo ""
echo "=== 启动 Web UI ==="
adb shell "su -c 'killall rustfrida-webui 2>/dev/null || true'"
adb shell "su -c 'nohup /data/adb/modules/rustfrida-kernelsu/bin/rustfrida-webui > /data/adb/rustfrida/logs/webui.log 2>&1 &'"

sleep 2

# 检查进程
if adb shell "ps -A | grep rustfrida-webui" > /dev/null; then
    echo "✅ Web UI 启动成功"
    echo ""
    echo "访问: http://localhost:8080"
    echo "或在手机浏览器: http://127.0.0.1:8080"
else
    echo "❌ Web UI 启动失败"
    echo "查看日志:"
    adb shell "su -c 'cat /data/adb/rustfrida/logs/webui.log'"
fi
