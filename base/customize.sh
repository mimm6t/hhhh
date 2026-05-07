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
}

set_permissions() {
  set_perm_recursive "$MODPATH" 0 0 0755 0644
  set_perm "$MODPATH/bin/rustfrida" 0 2000 0755 u:object_r:system_file:s0
}

print_modname
on_install
set_permissions
