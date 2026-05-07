/**
 * 钉钉助手 - 简化测试版本
 */

console.log("========================================");
console.log("[钉钉助手] 脚本开始加载");
console.log("========================================");

Java.perform(function() {
    console.log("[钉钉助手] Java.perform 开始执行");
    
    // 测试 1: Hook android.location.Location (一定存在)
    try {
        console.log("[测试] 尝试 Hook android.location.Location");
        var Location = Java.use("android.location.Location");
        
        var originalGetLatitude = Location.getLatitude;
        Location.getLatitude.implementation = function() {
            var result = originalGetLatitude.call(this);
            console.log("[Location] getLatitude() 被调用，原始值: " + result + " -> 修改为: 39.9042");
            return 39.9042;
        };
        
        var originalGetLongitude = Location.getLongitude;
        Location.getLongitude.implementation = function() {
            var result = originalGetLongitude.call(this);
            console.log("[Location] getLongitude() 被调用，原始值: " + result + " -> 修改为: 116.4074");
            return 116.4074;
        };
        
        console.log("[成功] android.location.Location Hook 成功");
    } catch (e) {
        console.log("[失败] android.location.Location Hook 失败: " + e);
    }
    
    // 测试 2: 尝试 Hook 高德地图
    try {
        console.log("[测试] 尝试 Hook com.amap.api.location.AMapLocation");
        var AMapLocation = Java.use("com.amap.api.location.AMapLocation");
        
        AMapLocation.getLatitude.implementation = function() {
            console.log("[高德] getLatitude() 被调用 -> 返回: 39.9042");
            return 39.9042;
        };
        
        AMapLocation.getLongitude.implementation = function() {
            console.log("[高德] getLongitude() 被调用 -> 返回: 116.4074");
            return 116.4074;
        };
        
        console.log("[成功] 高德地图 Hook 成功");
    } catch (e) {
        console.log("[失败] 高德地图 Hook 失败: " + e);
        console.log("[提示] 钉钉可能不使用高德地图，或类名不同");
    }
    
    // 测试 3: 列出所有加载的类（包含 location 关键字）
    try {
        console.log("[测试] 搜索包含 'location' 的类...");
        Java.enumerateLoadedClasses({
            onMatch: function(className) {
                if (className.toLowerCase().indexOf("location") !== -1) {
                    console.log("[发现] " + className);
                }
            },
            onComplete: function() {
                console.log("[完成] 类搜索完成");
            }
        });
    } catch (e) {
        console.log("[失败] 类搜索失败: " + e);
    }
    
    console.log("========================================");
    console.log("[钉钉助手] 初始化完成");
    console.log("[提示] 现在打开钉钉的打卡页面，触发定位请求");
    console.log("========================================");
});
