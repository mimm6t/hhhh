package com.hma.native

object HmaCore {
    external fun init(): Int
    external fun installHook(configJson: String): Int
    external fun uninstallHook(): Int
    external fun getStatus(): String
    external fun testWxshadow(): Boolean
    
    init {
        System.loadLibrary("hide_my_applist_rust")
    }
}
