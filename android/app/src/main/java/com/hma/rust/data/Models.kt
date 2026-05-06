package com.hma.rust.data

data class AppInfo(
    val packageName: String,
    val appName: String,
    val isHidden: Boolean = false
)

data class Config(
    val hiddenApps: List<String> = emptyList(),
    val templates: List<Template> = emptyList()
)

data class Template(
    val name: String,
    val apps: List<String>
)

data class LogEntry(
    val timestamp: Long,
    val level: String,
    val message: String
)