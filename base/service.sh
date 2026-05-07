#!/bin/sh
MODPATH=${0%/*}
exec 2> $MODPATH/logs/service.log
set -x
. $MODPATH/utils.sh || exit $?
wait_for_boot

# 启动 Web UI 服务（会自动启动 rustfrida server）
start_webui_server || exit $?
check_webui_is_up
