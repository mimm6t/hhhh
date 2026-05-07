#!/bin/sh
MODPATH=${0%/*}
exec 2> $MODPATH/logs/service.log
set -x
. $MODPATH/utils.sh || exit $?
wait_for_boot

# 启动 rustfrida server（用于 Hook 注入）
start_rustfrida_server
sleep 2
check_rustfrida_is_up

# 启动 Web UI 服务
start_webui_server || exit $?
check_webui_is_up

# 启动自动 Hook 监控
sh $MODPATH/bin/auto-hook.sh &
echo $! > $MODPATH/auto-hook.pid
