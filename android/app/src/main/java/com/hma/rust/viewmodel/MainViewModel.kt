package com.hma.rust.viewmodel

import androidx.lifecycle.ViewModel
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import com.hma.rust.data.*

class MainViewModel : ViewModel() {
    var config by mutableStateOf(Config())
        private set
    
    var apps by mutableStateOf<List<AppInfo>>(emptyList())
        private set
    
    var logs by mutableStateOf<List<LogEntry>>(emptyList())
        private set
    
    var isHookActive by mutableStateOf(false)
        private set
    
    fun toggleHook() {
        isHookActive = !isHookActive
    }
    
    fun hideApp(packageName: String) {
        config = config.copy(
            hiddenApps = config.hiddenApps + packageName
        )
    }
    
    fun showApp(packageName: String) {
        config = config.copy(
            hiddenApps = config.hiddenApps - packageName
        )
    }
    
    fun addTemplate(template: Template) {
        config = config.copy(
            templates = config.templates + template
        )
    }
    
    fun removeTemplate(templateName: String) {
        config = config.copy(
            templates = config.templates.filter { it.name != templateName }
        )
    }
    
    fun addLog(entry: LogEntry) {
        logs = logs + entry
    }
    
    fun clearLogs() {
        logs = emptyList()
    }
}