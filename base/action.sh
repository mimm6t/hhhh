#!/system/bin/sh
MODPATH=${0%/*}
PATH=$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin
exec 2> $MODPATH/logs/action.log
set -x
. $MODPATH/utils.sh || exit $?
[ -f $MODPATH/disable ] && { update_status "❌ (disabled)"; exit 0; }

# 停止 Web UI
[ -f "$MODPATH/webui.pid" ] && pid=$(cat "$MODPATH/webui.pid") && kill -0 "$pid" 2>/dev/null && kill -9 "$pid" && rm -f "$MODPATH/webui.pid"

# 启动 Web UI
start_webui_server || exit $?
sleep 1
check_webui_is_up 1
