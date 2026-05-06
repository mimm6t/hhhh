package com.hma.viewmodel

import android.app.Application
import android.content.pm.ApplicationInfo
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.hma.data.*
import com.hma.native.HmaCore
import com.hma.ui.LogEntry
import com.hma.ui.LogLevel
import com.hma.ui.LogManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

class MainViewModel(app: Application) : AndroidViewModel(app) {
    private val _hookStatus = MutableStateFlow(HookStatus())
    val hookStatus: StateFlow<HookStatus> = _hookStatus
    
    private val _apps = MutableStateFlow<List<AppInfo>>(emptyList())
    val apps: StateFlow<List<AppInfo>> = _apps
    
    private val _config = MutableStateFlow(Config())
    val config: StateFlow<Config> = _config
    
    private val _logs = MutableStateFlow<List<LogEntry>>(emptyList())
    val logs: StateFlow<List<LogEntry>> = _logs
    
    init {
        loadApps()
        checkStatus()
        loadConfig()
    }
    
    private fun loadApps() {
        viewModelScope.launch {
            val pm = getApplication<Application>().packageManager
            val packages = pm.getInstalledApplications(0)
            _apps.value = packages.map { info ->
                AppInfo(
                    packageName = info.packageName,
                    label = info.loadLabel(pm).toString(),
                    icon = info.loadIcon(pm),
                    isSystem = (info.flags and ApplicationInfo.FLAG_SYSTEM) != 0
                )
            }.sortedBy { it.label }
        }
    }
    
    private fun loadConfig() {
        // Load from storage
        _config.value = Config()
    }
    
    fun toggleHook() {
        viewModelScope.launch {
            if (_hookStatus.value.isActive) {
                HmaCore.uninstallHook()
                LogManager.info("Hook 已停用")
            } else {
                val json = configToJson(_config.value)
                HmaCore.installHook(json)
                LogManager.info("Hook 已启用")
            }
            checkStatus()
        }
    }
    
    fun checkStatus() {
        viewModelScope.launch {
            val status = HmaCore.getStatus()
            _hookStatus.value = HookStatus(isActive = status.contains("active"))
        }
    }
    
    fun toggleApp(app: AppInfo) {
        // Toggle app in hidden list
        LogManager.debug("切换应用: ${app.packageName}")
    }
    
    // Config management
    fun addScope(packageName: String) {
        _config.value = _config.value.copy(
            scope = _config.value.scope + (packageName to ScopeConfig(packageName))
        )
        LogManager.info("添加 Scope: $packageName")
    }
    
    fun removeScope(packageName: String) {
        _config.value = _config.value.copy(
            scope = _config.value.scope - packageName
        )
        LogManager.info("删除 Scope: $packageName")
    }
    
    fun editScope(packageName: String, config: ScopeConfig) {
        _config.value = _config.value.copy(
            scope = _config.value.scope + (packageName to config)
        )
        LogManager.debug("编辑 Scope: $packageName")
    }
    
    // Template management
    fun addTemplate(name: String, apps: Set<String>) {
        _config.value = _config.value.copy(
            templates = _config.value.templates + (name to Template(name, apps))
        )
        LogManager.info("添加模板: $name (${apps.size} 个应用)")
    }
    
    fun removeTemplate(name: String) {
        _config.value = _config.value.copy(
            templates = _config.value.templates - name
        )
        LogManager.info("删除模板: $name")
    }
    
    fun editTemplate(name: String, template: Template) {
        _config.value = _config.value.copy(
            templates = _config.value.templates + (name to template)
        )
        LogManager.debug("编辑模板: $name")
    }
    
    // Log management
    fun clearLogs() {
        LogManager.clear()
        refreshLogs()
    }
    
    fun refreshLogs() {
        _logs.value = LogManager.logs
    }
    
    // Settings
    fun setDetailLog(enabled: Boolean) {
        _config.value = _config.value.copy(detailLog = enabled)
        LogManager.info("详细日志: ${if (enabled) "开启" else "关闭"}")
        saveConfig()
    }
    
    fun setMaxLogSize(size: Int) {
        _config.value = _config.value.copy(maxLogSize = size)
        LogManager.debug("最大日志大小: $size KB")
        saveConfig()
    }
    
    fun exportConfig() {
        viewModelScope.launch {
            FileManager.exportConfig(
                getApplication(),
                _config.value,
                "/sdcard/Download/hma_config.json"
            ).onSuccess {
                LogManager.info("配置已导出到 /sdcard/Download/hma_config.json")
            }.onFailure {
                LogManager.error("导出失败: ${it.message}")
            }
        }
    }
    
    fun importConfig() {
        viewModelScope.launch {
            FileManager.importConfig(
                getApplication(),
                "/sdcard/Download/hma_config.json"
            ).onSuccess { config ->
                _config.value = config
                LogManager.info("配置已导入")
            }.onFailure {
                LogManager.error("导入失败: ${it.message}")
            }
        }
    }
    
    fun checkUpdate() {
        viewModelScope.launch {
            LogManager.info("检查更新...")
            UpdateManager.checkUpdate().onSuccess { update ->
                if (update != null) {
                    LogManager.info("发现新版本: ${update.version}")
                    // Show update dialog
                } else {
                    LogManager.info("已是最新版本")
                }
            }.onFailure {
                LogManager.error("检查更新失败: ${it.message}")
            }
        }
    }
    
    private fun saveConfig() {
        viewModelScope.launch {
            FileManager.saveConfig(getApplication(), _config.value)
        }
    }
    
    private fun loadConfig() {
        viewModelScope.launch {
            FileManager.loadConfig(getApplication()).onSuccess { config ->
                _config.value = config
            }
        }
    }
