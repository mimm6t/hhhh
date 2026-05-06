package com.hma.data

data class AppInfo(
    val packageName: String,
    val label: String,
    val icon: android.graphics.drawable.Drawable?,
    val isSystem: Boolean,
    val isHidden: Boolean = false
)

data class ScopeConfig(
    val packageName: String,
    val useWhitelist: Boolean = false,
    val excludeSystemApps: Boolean = true,
    val extraAppList: Set<String> = emptySet(),
    val applyTemplates: List<String> = emptyList()
)

data class Template(
    val name: String,
    val appList: Set<String>
)

data class Config(
    val configVersion: Int = 1,
    val detailLog: Boolean = false,
    val maxLogSize: Int = 1024,
    val scope: Map<String, ScopeConfig> = emptyMap(),
    val templates: Map<String, Template> = emptyMap()
)

data class HookStatus(
    val isActive: Boolean = false,
    val filterCount: Int = 0,
    val lastUpdate: Long = 0
)
