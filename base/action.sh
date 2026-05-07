#!/system/bin/sh
MODPATH=${0%/*}
PATH=$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin

exec 2> $MODPATH/logs/action.log
set -x

. $MODPATH/utils.sh || exit $?

[ -f $MODPATH/disable ] && {
    echo "[-] rustfrida-server is disabled"
    update_status "❌ (disabled)"
    sleep 1
    exit 0
}

# 停止现有进程
if [ -f "$MODPATH/rustfrida.pid" ]; then
    pid=$(cat "$MODPATH/rustfrida.pid")
    if kill -0 "$pid" 2>/dev/null; then
        echo "[-] Stopping rustfrida-server (PID: $pid)..."
        kill -9 "$pid"
    fi
    rm -f "$MODPATH/rustfrida.pid"
fi

# 启动
echo "[-] Starting rustfrida server..."
start_rustfrida_server || exit $?

sleep 1
check_rustfrida_is_up 1
