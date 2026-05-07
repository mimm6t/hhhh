/**
 * 钉钉助手 - Frida 版本
 * 功能: 虚拟定位、自动打卡
 */

console.log("[钉钉助手] 脚本加载中...");

// 配置
var Config = {
    enableVirtualLocation: true,
    latitude: 39.9042,
    longitude: 116.4074,
    address: "北京市东城区"
};

console.log("[配置]", JSON.stringify(Config));

Java.perform(function() {
    console.log("[钉钉助手] Java 环境就绪");
    
    if (!Config.enableVirtualLocation) {
        console.log("[定位] 虚拟定位未启用");
        return;
    }
    
    try {
        // Hook 高德地图 AMapLocation
        var AMapLocation = Java.use("com.amap.api.location.AMapLocation");
        
        AMapLocation.getLatitude.implementation = function() {
            console.log("[高德] getLatitude() -> " + Config.latitude);
            return Config.latitude;
        };
        
        AMapLocation.getLongitude.implementation = function() {
            console.log("[高德] getLongitude() -> " + Config.longitude);
            return Config.longitude;
        };
        
        AMapLocation.getAddress.implementation = function() {
            console.log("[高德] getAddress() -> " + Config.address);
            return Config.address;
        };
        
        console.log("[定位] 高德地图 Hook 成功");
    } catch (e) {
        console.log("[定位] 高德地图 Hook 失败:", e);
    }
    
    try {
        // Hook 百度地图 BDLocation
        var BDLocation = Java.use("com.baidu.location.BDLocation");
        
        BDLocation.getLatitude.implementation = function() {
            console.log("[百度] getLatitude() -> " + Config.latitude);
            return Config.latitude;
        };
        
        BDLocation.getLongitude.implementation = function() {
            console.log("[百度] getLongitude() -> " + Config.longitude);
            return Config.longitude;
        };
        
        BDLocation.getAddrStr.implementation = function() {
            console.log("[百度] getAddrStr() -> " + Config.address);
            return Config.address;
        };
        
        console.log("[定位] 百度地图 Hook 成功");
    } catch (e) {
        console.log("[定位] 百度地图 Hook 失败:", e);
    }
    
    try {
        // Hook Android 原生 Location
        var Location = Java.use("android.location.Location");
        
        Location.getLatitude.implementation = function() {
            var original = this.getLatitude();
            console.log("[原生] getLatitude() " + original + " -> " + Config.latitude);
            return Config.latitude;
        };
        
        Location.getLongitude.implementation = function() {
            var original = this.getLongitude();
            console.log("[原生] getLongitude() " + original + " -> " + Config.longitude);
            return Config.longitude;
        };
        
        console.log("[定位] Android 原生 Location Hook 成功");
    } catch (e) {
        console.log("[定位] Android 原生 Location Hook 失败:", e);
    }
    
    try {
        // Hook LocationManager
        var LocationManager = Java.use("android.location.LocationManager");
        
        LocationManager.getLastKnownLocation.overload('java.lang.String').implementation = function(provider) {
            console.log("[LocationManager] getLastKnownLocation(" + provider + ")");
            var location = this.getLastKnownLocation(provider);
            if (location != null) {
                location.setLatitude(Config.latitude);
                location.setLongitude(Config.longitude);
                console.log("[LocationManager] 修改为虚拟位置");
            }
            return location;
        };
        
        console.log("[定位] LocationManager Hook 成功");
    } catch (e) {
        console.log("[定位] LocationManager Hook 失败:", e);
    }
    
    console.log("[钉钉助手] 初始化完成");
    console.log("[钉钉助手] 虚拟定位: 已启用");
    console.log("[钉钉助手] 目标位置: " + Config.address + " (" + Config.latitude + ", " + Config.longitude + ")");
});
