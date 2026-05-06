package com.hma

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.hma.ui.AppListScreen
import com.hma.ui.MainScreen
import com.hma.viewmodel.MainViewModel

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            HMATheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    HMAApp()
                }
            }
        }
    }
}

@Composable
fun HMAApp() {
    val navController = rememberNavController()
    val viewModel: MainViewModel = viewModel()
    val hookStatus by viewModel.hookStatus.collectAsState()
    val apps by viewModel.apps.collectAsState()
    val config by viewModel.config.collectAsState()
    val logs by viewModel.logs.collectAsState()
    var showAboutDialog by remember { mutableStateOf(false) }
    
    NavHost(navController, startDestination = "main") {
        composable("main") {
            MainScreen(
                hookStatus = hookStatus,
                onToggleHook = { viewModel.toggleHook() },
                onNavigateToApps = { navController.navigate("apps") },
                onNavigateToConfig = { navController.navigate("config") },
                onNavigateToTemplates = { navController.navigate("templates") },
                onNavigateToLogs = { navController.navigate("logs") },
                onNavigateToSettings = { navController.navigate("settings") }
            )
        }
        
        composable("apps") {
            AppListScreen(
                apps = apps,
                onAppToggle = { viewModel.toggleApp(it) },
                onBack = { navController.popBackStack() }
            )
        }
        
        composable("config") {
            ConfigScreen(
                scopes = config.scope,
                onAddScope = { viewModel.addScope(it) },
                onRemoveScope = { viewModel.removeScope(it) },
                onEditScope = { pkg, cfg -> viewModel.editScope(pkg, cfg) },
                onBack = { navController.popBackStack() }
            )
        }
        
        composable("templates") {
            TemplateScreen(
                templates = config.templates,
                onAddTemplate = { name, apps -> viewModel.addTemplate(name, apps) },
                onRemoveTemplate = { viewModel.removeTemplate(it) },
                onEditTemplate = { name, tpl -> viewModel.editTemplate(name, tpl) },
                onBack = { navController.popBackStack() }
            )
        }
        
        composable("logs") {
            LogScreen(
                logs = logs,
                onClearLogs = { viewModel.clearLogs() },
                onRefresh = { viewModel.refreshLogs() },
                onBack = { navController.popBackStack() }
            )
        }
        
        composable("settings") {
            SettingsScreen(
                detailLog = config.detailLog,
                maxLogSize = config.maxLogSize,
                onDetailLogChange = { viewModel.setDetailLog(it) },
                onMaxLogSizeChange = { viewModel.setMaxLogSize(it) },
                onExportConfig = { viewModel.exportConfig() },
                onImportConfig = { viewModel.importConfig() },
                onCheckUpdate = { viewModel.checkUpdate() },
                onAbout = { showAboutDialog = true },
                onBack = { navController.popBackStack() }
            )
        }
    }
    
    if (showAboutDialog) {
        AboutDialog(onDismiss = { showAboutDialog = false })
    }
}

@Composable
fun HMATheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = darkColorScheme(),
        content = content
    )
}
