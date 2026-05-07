#!/bin/sh
MODPATH=${0%/*}
RUSTFRIDA_BIN="$MODPATH/bin/rustfrida"
PATH="$MODPATH/bin:$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin"

exec 2> $MODPATH/logs/utils.log
set -x

start_rustfrida_server() {
  if [ ! -x "$RUSTFRIDA_BIN" ]; then
    echo "[-] rustfrida binary not found: $RUSTFRIDA_BIN"
    update_status "❌ (missing binary)"
    return 1
  fi

  # 读取配置
  [ -f "$MODPATH/config/server.conf" ] && . "$MODPATH/config/server.conf"
  RPC_PORT=${RPC_PORT:-27042}
  LISTEN_ADDR=${LISTEN_ADDR:-0.0.0.0}
  
  # 启动 server 模式
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
    if [ -f "$MODPATH/rustfrida.pid" ]; then
      pid=$(cat "$MODPATH/rustfrida.pid")
      if kill -0 "$pid" 2>/dev/null; then
        echo "[-] rustfrida server is running (PID: $pid) 💉😜"
        update_status "✅ (active)"
        return 0
      fi
    fi
    echo "[-] Checking rustfrida status: $counter"
    counter=$((counter + 1))
    sleep 1.5
  done

  update_status "❌ (failed)"
  return 1
}

update_status() {
  string="description=Run rustfrida-server on boot: $1"
  sed -i "s/^description=.*/$string/g" "$MODPATH/module.prop"
}

wait_for_boot() {
  while true; do
    result="$(getprop sys.boot_completed)"
    [ $? -ne 0 ] && exit 1
    [ "$result" = "1" ] && break
    sleep 3
  done
}
