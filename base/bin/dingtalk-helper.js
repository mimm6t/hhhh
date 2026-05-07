/**
 * 钉钉助手 - Frida 版本
 * 功能: 虚拟定位、自动打卡
 * 
 * 使用方法:
 * 1. 在 rustFrida Manager 中启用此脚本
 * 2. 在 Web UI 配置定位和打卡参数
 * 3. 启动钉钉应用
 */

console.log("[钉钉助手] 脚本加载中...");

// ==================== 配置管理 ====================
var Config = {
    // 虚拟定位
    enableVirtualLocation: false,
    latitude: 39.9042,  // 北京天安门
    longitude: 116.4074,
    address: "北京市东城区",
    
    // 自动打卡
    enableAutoCheckIn: false,
    checkInDelay: 3000,  // 延迟3秒
    
    // 加载配置
    load: function() {
        try {
            var prefs = Java.use("android.preference.PreferenceManager")
                .getDefaultSharedPreferences(Java.use("android.app.ActivityThread")
                    .currentApplication().getApplicationContext());
            
            this.enableVirtualLocation = prefs.getBoolean("dd_enable_virtual_location", false);
            this.latitude = parseFloat(prefs.getString("dd_location_latitude", "39.9042"));
            this.longitude = parseFloat(prefs.getString("dd_location_longitude", "116.4074"));
            this.address = prefs.getString("dd_location_address", "北京市东城区");
            this.enableAutoCheckIn = prefs.getBoolean("dd_enable_auto_checkin", false);
            this.checkInDelay = prefs.getInt("dd_checkin_delay", 3000);
            
            console.log("[配置] 加载成功:", JSON.stringify(this, null, 2));
        } catch (e) {
            console.log("[配置] 加载失败，使用默认配置:", e);
        }
    },
    
    // 保存配置
    save: function() {
        try {
            var prefs = Java.use("android.preference.PreferenceManager")
                .getDefaultSharedPreferences(Java.use("android.app.ActivityThread")
                    .currentApplication().getApplicationContext());
            var editor = prefs.edit();
            
            editor.putBoolean("dd_enable_virtual_location", this.enableVirtualLocation);
            editor.putString("dd_location_latitude", this.latitude.toString());
            editor.putString("dd_location_longitude", this.longitude.toString());
            editor.putString("dd_location_address", this.address);
            editor.putBoolean("dd_enable_auto_checkin", this.enableAutoCheckIn);
            editor.putInt("dd_checkin_delay", this.checkInDelay);
            editor.apply();
            
            console.log("[配置] 保存成功");
        } catch (e) {
            console.log("[配置] 保存失败:", e);
        }
    }
};

// ==================== 定位模块 ====================
var LocationModule = {
    hooked: false,
    
    init: function() {
        if (this.hooked) return;
        
        try {
            // Hook android.location.Location
            var Location = Java.use("android.location.Location");
            
            Location.getLatitude.implementation = function() {
                var original = this.getLatitude();
                if (Config.enableVirtualLocation) {
                    console.log("[定位] 修改纬度:", original, "->", Config.latitude);
                    return Config.latitude;
                }
                return original;
            };
            
            Location.getLongitude.implementation = function() {
                var original = this.getLongitude();
                if (Config.enableVirtualLocation) {
                    console.log("[定位] 修改经度:", original, "->", Config.longitude);
                    return Config.longitude;
                }
                return original;
            };
            
            Location.setLatitude.implementation = function(lat) {
                if (Config.enableVirtualLocation) {
                    console.log("[定位] 拦截设置纬度:", lat, "-> 使用虚拟:", Config.latitude);
                    this.setLatitude(Config.latitude);
                } else {
                    this.setLatitude(lat);
                }
            };
            
            Location.setLongitude.implementation = function(lon) {
                if (Config.enableVirtualLocation) {
                    console.log("[定位] 拦截设置经度:", lon, "-> 使用虚拟:", Config.longitude);
                    this.setLongitude(Config.longitude);
                } else {
                    this.setLongitude(lon);
                }
            };
            
            console.log("[定位] Hook 成功");
            this.hooked = true;
        } catch (e) {
            console.log("[定位] Hook 失败:", e);
        }
    }
};

// ==================== 打卡模块 ====================
var CheckInModule = {
    hooked: false,
    
    init: function() {
        if (this.hooked) return;
        
        try {
            // 尝试 Hook 打卡相关 Activity
            this.hookCheckInActivity();
            this.hookAttendanceActivity();
            
            console.log("[打卡] Hook 成功");
            this.hooked = true;
        } catch (e) {
            console.log("[打卡] Hook 失败:", e);
        }
    },
    
    hookCheckInActivity: function() {
        try {
            // 钉钉打卡页面可能的类名
            var activityNames = [
                "com.alibaba.android.dingtalk.biz.attendance.CheckInActivity",
                "com.alibaba.android.dingtalk.biz.attendance.ui.CheckInActivity",
                "com.alibaba.lightapp.runtime.activity.CommonWebViewActivity"
            ];
            
            for (var i = 0; i < activityNames.length; i++) {
                try {
                    var Activity = Java.use(activityNames[i]);
                    
                    Activity.onResume.implementation = function() {
                        console.log("[打卡] 检测到打卡页面:", activityNames[i]);
                        this.onResume();
                        
                        if (Config.enableAutoCheckIn) {
                            var self = this;
                            setTimeout(function() {
                                CheckInModule.performAutoCheckIn(self);
                            }, Config.checkInDelay);
                        }
                    };
                    
                    console.log("[打卡] Hook Activity 成功:", activityNames[i]);
                    break;
                } catch (e) {
                    // 尝试下一个
                }
            }
        } catch (e) {
            console.log("[打卡] Hook Activity 失败:", e);
        }
    },
    
    hookAttendanceActivity: function() {
        try {
            // Hook 考勤相关的 Fragment
            var Fragment = Java.use("android.support.v4.app.Fragment");
            
            Fragment.onResume.implementation = function() {
                var className = this.getClass().getName();
                if (className.indexOf("Attendance") >= 0 || className.indexOf("CheckIn") >= 0) {
                    console.log("[打卡] 检测到考勤 Fragment:", className);
                }
                this.onResume();
            };
        } catch (e) {
            // Fragment 可能不存在
        }
    },
    
    performAutoCheckIn: function(activity) {
        try {
            console.log("[打卡] 尝试自动打卡...");
            
            // 方法1: 查找打卡按钮并点击
            Java.scheduleOnMainThread(function() {
                try {
                    var rootView = activity.getWindow().getDecorView();
                    CheckInModule.findAndClickButton(rootView);
                } catch (e) {
                    console.log("[打卡] 自动点击失败:", e);
                }
            });
        } catch (e) {
            console.log("[打卡] 执行失败:", e);
        }
    },
    
    findAndClickButton: function(view) {
        try {
            var View = Java.use("android.view.View");
            var ViewGroup = Java.use("android.view.ViewGroup");
            var Button = Java.use("android.widget.Button");
            var TextView = Java.use("android.widget.TextView");
            
            // 检查当前 View
            if (view.isClickable()) {
                var text = "";
                try {
                    if (Button.class.isInstance(view) || TextView.class.isInstance(view)) {
                        text = view.getText().toString();
                    }
                } catch (e) {}
                
                // 查找包含"打卡"的按钮
                if (text.indexOf("打卡") >= 0 || text.indexOf("签到") >= 0) {
                    console.log("[打卡] 找到打卡按钮:", text);
                    view.performClick();
                    return true;
                }
            }
            
            // 递归查找子 View
            if (ViewGroup.class.isInstance(view)) {
                var childCount = view.getChildCount();
                for (var i = 0; i < childCount; i++) {
                    var child = view.getChildAt(i);
                    if (this.findAndClickButton(child)) {
                        return true;
                    }
                }
            }
        } catch (e) {
            console.log("[打卡] 查找按钮失败:", e);
        }
        return false;
    }
};

// ==================== RPC 接口 ====================
rpc.exports = {
    // 获取配置
    getConfig: function() {
        return {
            enableVirtualLocation: Config.enableVirtualLocation,
            latitude: Config.latitude,
            longitude: Config.longitude,
            address: Config.address,
            enableAutoCheckIn: Config.enableAutoCheckIn,
            checkInDelay: Config.checkInDelay
        };
    },
    
    // 设置配置
    setConfig: function(config) {
        if (config.enableVirtualLocation !== undefined) {
            Config.enableVirtualLocation = config.enableVirtualLocation;
        }
        if (config.latitude !== undefined) {
            Config.latitude = config.latitude;
        }
        if (config.longitude !== undefined) {
            Config.longitude = config.longitude;
        }
        if (config.address !== undefined) {
            Config.address = config.address;
        }
        if (config.enableAutoCheckIn !== undefined) {
            Config.enableAutoCheckIn = config.enableAutoCheckIn;
        }
        if (config.checkInDelay !== undefined) {
            Config.checkInDelay = config.checkInDelay;
        }
        
        Config.save();
        console.log("[RPC] 配置已更新:", JSON.stringify(Config, null, 2));
        return { success: true };
    },
    
    // 设置虚拟定位
    setLocation: function(lat, lon, addr) {
        Config.latitude = lat;
        Config.longitude = lon;
        Config.address = addr || "";
        Config.save();
        console.log("[RPC] 定位已更新:", lat, lon, addr);
        return { success: true };
    },
    
    // 启用/禁用虚拟定位
    toggleVirtualLocation: function(enable) {
        Config.enableVirtualLocation = enable;
        Config.save();
        console.log("[RPC] 虚拟定位:", enable ? "已启用" : "已禁用");
        return { success: true };
    },
    
    // 启用/禁用自动打卡
    toggleAutoCheckIn: function(enable) {
        Config.enableAutoCheckIn = enable;
        Config.save();
        console.log("[RPC] 自动打卡:", enable ? "已启用" : "已禁用");
        return { success: true };
    },
    
    // 手动触发打卡
    triggerCheckIn: function() {
        console.log("[RPC] 手动触发打卡");
        Java.perform(function() {
            try {
                var ActivityThread = Java.use("android.app.ActivityThread");
                var currentActivity = ActivityThread.currentActivity();
                if (currentActivity) {
                    CheckInModule.performAutoCheckIn(currentActivity);
                    return { success: true };
                } else {
                    return { success: false, error: "未找到当前 Activity" };
                }
            } catch (e) {
                return { success: false, error: e.toString() };
            }
        });
    }
};

// ==================== 主初始化 ====================
Java.perform(function() {
    console.log("[钉钉助手] Java 环境就绪");
    
    // 加载配置
    Config.load();
    
    // 初始化模块
    LocationModule.init();
    CheckInModule.init();
    
    console.log("[钉钉助手] 初始化完成");
    console.log("[钉钉助手] 虚拟定位:", Config.enableVirtualLocation ? "已启用" : "已禁用");
    console.log("[钉钉助手] 自动打卡:", Config.enableAutoCheckIn ? "已启用" : "已禁用");
});
