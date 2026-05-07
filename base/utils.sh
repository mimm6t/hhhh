#!/bin/sh
MODPATH=${0%/*}
RUSTFRIDA_BIN="$MODPATH/bin/rustfrida"
WEBUI_BIN="$MODPATH/bin/rustfrida-webui"
PATH="$MODPATH/bin:$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin"

exec 2> $MODPATH/logs/utils.log
set -x

start_webui_server() {
  echo "[$(date)] Starting Web UI server..."
  [ ! -x "$WEBUI_BIN" ] && { echo "[-] rustfrida-webui not found at: $WEBUI_BIN"; update_status "❌ (missing webui)"; return 1; }
  [ ! -x "$RUSTFRIDA_BIN" ] && { echo "[-] rustfrida binary not found at: $RUSTFRIDA_BIN"; update_status "❌ (missing binary)"; return 1; }
  
  echo "[+] Found rustfrida-webui: $(ls -lh $WEBUI_BIN)"
  echo "[+] Starting Web UI on 0.0.0.0:8080..."
  
  # 启动 Rust Web UI
  "$WEBUI_BIN" > "$MODPATH/logs/webui.log" 2>&1 &
  echo $! > "$MODPATH/webui.pid"
  echo "[+] Web UI started with PID: $(cat $MODPATH/webui.pid)"
}

check_webui_is_up() {
  timeout=${1:-4}
  counter=0
  echo "[$(date)] Checking if Web UI is up (timeout: ${timeout}s)..."
  while [ $counter -lt $timeout ]; do
    if [ -f "$MODPATH/webui.pid" ]; then
      pid=$(cat "$MODPATH/webui.pid")
      if kill -0 "$pid" 2>/dev/null; then
        echo "[+] Web UI running (PID: $pid)"
        update_status "✅ (active - WebUI:8080)"
        return 0
      else
        echo "[-] PID $pid not running"
      fi
    else
      echo "[-] webui.pid not found"
    fi
    counter=$((counter + 1))
    sleep 1.5
  done
  echo "[-] Web UI failed to start"
  echo "[-] Checking webui.log:"
  [ -f "$MODPATH/logs/webui.log" ] && tail -20 "$MODPATH/logs/webui.log"
  update_status "❌ (failed)"
  return 1
}

start_rustfrida_server() {
  echo "[$(date)] Starting rustfrida server..."
  [ ! -x "$RUSTFRIDA_BIN" ] && { echo "[-] rustfrida binary not found"; update_status "❌ (missing binary)"; return 1; }
  [ -f "$MODPATH/config/server.conf" ] && . "$MODPATH/config/server.conf"
  RPC_PORT=${RPC_PORT:-27042}
  LISTEN_ADDR=${LISTEN_ADDR:-0.0.0.0}
  ARGS="--server --rpc-port $LISTEN_ADDR:$RPC_PORT"
  [ -n "$PROFILE" ] && ARGS="$ARGS --profile $PROFILE"
  [ -n "$VERBOSE" ] && ARGS="$ARGS --verbose"
  
  echo "[+] Starting: $RUSTFRIDA_BIN $ARGS"
  
  # 使用 setsid 和 nohup 确保后台运行
  setsid nohup "$RUSTFRIDA_BIN" $ARGS > "$MODPATH/logs/rustfrida.log" 2>&1 < /dev/null &
  echo $! > "$MODPATH/rustfrida.pid"
  echo "[+] rustfrida PID: $(cat $MODPATH/rustfrida.pid)"
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
