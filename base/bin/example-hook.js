// rustFrida Hook 示例脚本
// 这个脚本会在应用启动时自动加载

console.log("[*] Hook script loaded");

Java.ready(function() {
    console.log("[*] Java runtime ready");
    console.log("[*] Package:", Java.use("android.app.ActivityThread")
        .currentApplication().getApplicationContext().getPackageName());
    
    // 示例 1: Hook Activity.onCreate
    var Activity = Java.use("android.app.Activity");
    Activity.onCreate.overload("android.os.Bundle").implementation = function(bundle) {
        console.log("[Hook] Activity.onCreate:", this.getClass().getName());
        this.onCreate(bundle);
    };
    
    // 示例 2: Hook Toast
    var Toast = Java.use("android.widget.Toast");
    Toast.makeText.overload("android.content.Context", "java.lang.CharSequence", "int").implementation = function(ctx, text, duration) {
        console.log("[Hook] Toast.makeText:", text.toString());
        return this.makeText(ctx, text, duration);
    };
    
    // 示例 3: Hook Log
    var Log = Java.use("android.util.Log");
    Log.i.overload("java.lang.String", "java.lang.String").implementation = function(tag, msg) {
        console.log("[Hook] Log.i:", tag, msg);
        return this.i(tag, msg);
    };
    
    console.log("[*] Hooks installed successfully");
});

// RPC 导出（可选）
rpc.exports = {
    test: function() {
        return "Hook is working!";
    },
    getPackage: function() {
        var result = "";
        Java.ready(function() {
            result = Java.use("android.app.ActivityThread")
                .currentApplication().getApplicationContext().getPackageName();
        });
        return result;
    }
};
