#!/bin/sh
SKIPMOUNT=true
PROPFILE=false
POSTFSDATA=false
LATESTARTSERVICE=true
REPLACE=""
SKIPUNZIP=1

print_modname() {
  ui_print " "
  ui_print "    ********************************************"
  ui_print "    *       rustFrida for KernelSU/Magisk      *"
  ui_print "    ********************************************"
  ui_print " "
}

on_install() {
  [ "$ARCH" != "arm64" ] && abort "! Unsupported arch: $ARCH (arm64 only)"
  ui_print "- Architecture: $ARCH"
  
  if [ "$KSU" = true ]; then
    ui_print "- KernelSU detected"
  elif [ -n "$MAGISK_VER_CODE" ]; then
    ui_print "- Magisk detected"
  else
    abort "! Install from KernelSU/Magisk app only"
  fi
  
  unzip -oq "$ZIPFILE" -x "META-INF/*" "files/*" -d "$MODPATH" || abort "! Extract failed"
  rm -rf "$MODPATH/files" "$MODPATH/system"
  touch "$MODPATH/skip_mount"
  mkdir -p "$MODPATH/bin" "$MODPATH/logs"
  unzip -ojq "$ZIPFILE" "files/rustfrida" -d "$MODPATH/bin" || abort "! Extract rustfrida failed"
  unzip -ojq "$ZIPFILE" "files/rustfrida-webui" -d "$MODPATH/bin" || abort "! Extract rustfrida-webui failed"
  ui_print "- Extracted rustfrida and rustfrida-webui"
  
  # 创建 rustfrida 数据目录
  mkdir -p /data/adb/rustfrida/scripts
  mkdir -p /data/adb/rustfrida/logs
  
  # 复制示例脚本（如果不存在）
  if [ ! -f /data/adb/rustfrida/scripts/example-hook.js ]; then
    cp -f "$MODPATH/bin/example-hook.js" /data/adb/rustfrida/scripts/
    ui_print "- 已安装示例脚本: example-hook.js"
  fi
  
  if [ ! -f /data/adb/rustfrida/scripts/dingtalk-helper.js ]; then
    cp -f "$MODPATH/bin/dingtalk-helper.js" /data/adb/rustfrida/scripts/
    ui_print "- 已安装钉钉助手: dingtalk-helper.js"
  fi
  
  # 初始化配置文件
  if [ ! -f /data/adb/rustfrida/hooks.json ]; then
    echo '{"enabled":[],"hooks":{}}' > /data/adb/rustfrida/hooks.json
  fi
  
  ui_print "- Web UI 端口: 8080"
  ui_print "- 使用 adb forward tcp:8080 tcp:8080 访问"
}

set_permissions() {
  set_perm_recursive "$MODPATH" 0 0 0755 0644
  set_perm "$MODPATH/bin/rustfrida" 0 2000 0755 u:object_r:system_file:s0
  set_perm "$MODPATH/bin/rustfrida-webui" 0 2000 0755 u:object_r:system_file:s0
}

print_modname
on_install
set_permissions
