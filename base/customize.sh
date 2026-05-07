#!/bin/sh
SKIPMOUNT=true
PROPFILE=false
POSTFSDATA=false
LATESTARTSERVICE=true
REPLACE=""

[ ! -d "$MODPATH/logs" ] && mkdir -p "$MODPATH/logs"

PATH="$PATH:/data/adb/ap/bin:/data/adb/magisk:/data/adb/ksu/bin"
SKIPUNZIP=1

print_modname() {
  ui_print " "
  ui_print "    ********************************************"
  ui_print "    *          Magisk/KernelSU/APatch          *"
  ui_print "    *              rustFrida                   *"
  ui_print "    ********************************************"
  ui_print " "
}

on_install() {
  case $ARCH in
    arm64) F_ARCH=$ARCH;;
    *)     abort "! Unsupported architecture: $ARCH (rustFrida only supports arm64)";;
  esac

  ui_print "- Detected architecture: $F_ARCH"

  if [ "$BOOTMODE" = true ] && [ "$KSU" = true ]; then
      ui_print "- Installing from KernelSU"
      ui_print "- KernelSU version: $KSU_KERNEL_VER_CODE (kernel) + $KSU_VER_CODE (ksud)"
  elif [ "$BOOTMODE" = true ] && [ -n "$APATCH" ]; then
      ui_print "- Installing from APatch"
      ui_print "- APatch version: $APATCH_VER_CODE. Magisk version: $MAGISK_VER_CODE"
  elif [ "$BOOTMODE" = true ] && [ -n "$MAGISK_VER_CODE" ]; then
      ui_print "- Installing from Magisk"
      ui_print "- Magisk version: $MAGISK_VER_CODE ($MAGISK_VER)"
  else
    ui_print "*********************************************************"
    ui_print "! Install from recovery is not supported"
    ui_print "! Please install from KernelSU or Magisk app"
    abort "*********************************************************"
  fi

  ui_print "- Unzipping module files..."
  unzip -oq "$ZIPFILE" -x "META-INF/*" "files/*" -d "$MODPATH" \
    || abort "! Failed to extract module files"

  rm -rf "$MODPATH/files" "$MODPATH/system/bin"
  rmdir "$MODPATH/system" 2>/dev/null

  touch "$MODPATH/skip_mount" || abort "! Failed to create skip_mount marker"

  F_BINDIR="$MODPATH/bin"
  mkdir -p "$F_BINDIR" || abort "! Failed to create module bin directory"

  ui_print "- Installing rustfrida to module bin..."
  unzip -ojq "$ZIPFILE" "files/rustfrida" -d "$F_BINDIR" \
    || abort "! Failed to extract rustfrida binary"
}

set_permissions() {
  set_perm_recursive "$MODPATH" 0 0 0755 0644 \
    || abort "! Failed to set default permissions"

  set_perm "$MODPATH/bin/rustfrida" 0 2000 0755 u:object_r:system_file:s0 \
    || abort "! Failed to set rustfrida binary permissions"
}

exec 3>&2 2>"$MODPATH/logs/custom.log"
set -x

print_modname
on_install
set_permissions

set +x
exec 2>&3 3>&-

[ -f "$MODPATH/disable" ] && {
  string="description=Run rustfrida-server on boot: ❌ (failed)"
  sed -i "s/^description=.*/$string/g" "$MODPATH/module.prop"
}

return 0
