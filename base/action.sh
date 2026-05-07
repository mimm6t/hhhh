#!/system/bin/sh
MODPATH=${0%/*}
PATH=$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin
exec 2> $MODPATH/logs/action.log
set -x
. $MODPATH/utils.sh || exit $?
[ -f $MODPATH/disable ] && { update_status "❌ (disabled)"; exit 0; }
[ -f "$MODPATH/rustfrida.pid" ] && pid=$(cat "$MODPATH/rustfrida.pid") && kill -0 "$pid" 2>/dev/null && kill -9 "$pid" && rm -f "$MODPATH/rustfrida.pid"
start_rustfrida_server || exit $?
sleep 1
check_rustfrida_is_up 1
