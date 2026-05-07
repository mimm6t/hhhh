package com.hma.rust.native

object HmaCore {
    init {
        System.loadLibrary("hide_my_applist_rust")
    }
    
    external fun init(): Int
    external fun installHook(configJson: String): Int
    external fun uninstallHook(): Int
    external fun getStatus(): String
    external fun testFrida(): Boolean
}