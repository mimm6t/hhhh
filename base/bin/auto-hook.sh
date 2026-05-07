#!/system/bin/sh
# 自动 Hook 监控脚本
# 监听应用启动并自动注入 Frida 脚本

MODPATH="/data/adb/modules/rustfrida-kernelsu"
RUSTFRIDA="$MODPATH/bin/rustfrida"
HOOKS_CONFIG="/data/adb/rustfrida/hooks.json"
SCRIPTS_DIR="/data/adb/rustfrida/scripts"

log() {
    echo "[$(date '+%H:%M:%S')] $*" >> "$MODPATH/logs/auto-hook.log"
}

log "Auto-hook monitor started"

# 读取配置
if [ ! -f "$HOOKS_CONFIG" ]; then
    log "No hooks.json found"
    exit 0
fi

# 持续监控
while true; do
    # 读取启用的包名
    enabled=$(cat "$HOOKS_CONFIG" | grep -o '"enabled":\[[^]]*\]' | sed 's/"enabled":\[//;s/\]//;s/"//g' | tr ',' '\n')
    
    for package in $enabled; do
        # 检查应用是否在运行
        pid=$(pidof "$package" 2>/dev/null)
        
        if [ -n "$pid" ]; then
            # 检查是否已经注入
            if ! grep -q "$package" "$MODPATH/logs/injected.log" 2>/dev/null; then
                log "Detected $package (PID: $pid), injecting..."
                
                # 获取对应的脚本
                script=$(cat "$HOOKS_CONFIG" | grep -A1 "\"$package\"" | tail -1 | sed 's/.*: "//;s/".*//')
                
                if [ -n "$script" ] && [ -f "$SCRIPTS_DIR/$script" ]; then
                    # Spawn 模式注入
                    "$RUSTFRIDA" --spawn "$package" -l "$SCRIPTS_DIR/$script" >> "$MODPATH/logs/rustfrida.log" 2>&1 &
                    
                    echo "$package" >> "$MODPATH/logs/injected.log"
                    log "Injected $script into $package"
                else
                    log "Script not found: $script"
                fi
            fi
        else
            # 应用未运行，清除注入标记
            sed -i "/$package/d" "$MODPATH/logs/injected.log" 2>/dev/null
        fi
    done
    
    sleep 5
done
