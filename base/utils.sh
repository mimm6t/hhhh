#!/bin/sh
MODPATH=${0%/*}
RUSTFRIDA_BIN="$MODPATH/bin/rustfrida"
WEBUI_SCRIPT="$MODPATH/bin/webui.py"
PATH="$MODPATH/bin:$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin"

exec 2> $MODPATH/logs/utils.log
set -x

start_webui_server() {
  [ ! -f "$WEBUI_SCRIPT" ] && { echo "[-] webui.py not found"; update_status "❌ (missing webui)"; return 1; }
  [ ! -x "$RUSTFRIDA_BIN" ] && { echo "[-] rustfrida binary not found"; update_status "❌ (missing binary)"; return 1; }
  
  # 启动 Web UI（会自动启动 rustfrida）
  python3 "$WEBUI_SCRIPT" > "$MODPATH/logs/webui.log" 2>&1 &
  echo $! > "$MODPATH/webui.pid"
}

check_webui_is_up() {
  timeout=${1:-4}
  counter=0
  while [ $counter -lt $timeout ]; do
    [ -f "$MODPATH/webui.pid" ] && pid=$(cat "$MODPATH/webui.pid") && kill -0 "$pid" 2>/dev/null && { echo "[-] Web UI running (PID: $pid)"; update_status "✅ (active - WebUI:8080)"; return 0; }
    counter=$((counter + 1))
    sleep 1.5
  done
  update_status "❌ (failed)"
  return 1
}

start_rustfrida_server() {
  [ ! -x "$RUSTFRIDA_BIN" ] && { echo "[-] rustfrida binary not found"; update_status "❌ (missing binary)"; return 1; }
  [ -f "$MODPATH/config/server.conf" ] && . "$MODPATH/config/server.conf"
  RPC_PORT=${RPC_PORT:-27042}
  LISTEN_ADDR=${LISTEN_ADDR:-0.0.0.0}
  ARGS="--server --rpc-port $LISTEN_ADDR:$RPC_PORT"
  [ -n "$PROFILE" ] && ARGS="$ARGS --profile $PROFILE"
  [ -n "$VERBOSE" ] && ARGS="$ARGS --verbose"
  "$RUSTFRIDA_BIN" $ARGS > "$MODPATH/logs/rustfrida.log" 2>&1 &
  echo $! > "$MODPATH/rustfrida.pid"
}

check_rustfrida_is_up() {
  timeout=${1:-4}
  counter=0
  while [ $counter -lt $timeout ]; do
    [ -f "$MODPATH/rustfrida.pid" ] && pid=$(cat "$MODPATH/rustfrida.pid") && kill -0 "$pid" 2>/dev/null && { echo "[-] rustfrida running (PID: $pid)"; update_status "✅ (active)"; return 0; }
    counter=$((counter + 1))
    sleep 1.5
  done
  update_status "❌ (failed)"
  return 1
}

update_status() {
  sed -i "s/^description=.*/description=Run rustfrida-server on boot: $1/g" "$MODPATH/module.prop"
}

wait_for_boot() {
  while true; do
    result="$(getprop sys.boot_completed)"
    [ $? -ne 0 ] && exit 1
    [ "$result" = "1" ] && break
    sleep 3
  done
}
