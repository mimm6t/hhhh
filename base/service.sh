#!/bin/sh
MODPATH=${0%/*}

exec 2> $MODPATH/logs/service.log
set -x

. $MODPATH/utils.sh || exit $?

wait_for_boot

start_rustfrida_server || exit $?

check_rustfrida_is_up
