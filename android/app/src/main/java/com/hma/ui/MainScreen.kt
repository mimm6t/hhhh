package com.hma.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.hma.data.HookStatus

@Composable
fun MainScreen(
    hookStatus: HookStatus,
    onToggleHook: () -> Unit,
    onNavigateToApps: () -> Unit,
    onNavigateToConfig: () -> Unit,
    onNavigateToTemplates: () -> Unit,
    onNavigateToLogs: () -> Unit,
    onNavigateToSettings: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        StatusCard(hookStatus, onToggleHook)
        
        Button(
            onClick = onNavigateToApps,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("应用管理")
        }
        
        Button(
            onClick = onNavigateToConfig,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("配置管理")
        }
        
        Button(
            onClick = onNavigateToTemplates,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("模板管理")
        }
        
        Button(
            onClick = onNavigateToLogs,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("日志查看")
        }
        
        Button(
            onClick = onNavigateToSettings,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("设置")
        }
    }
}

@Composable
fun StatusCard(status: HookStatus, onToggle: () -> Unit) {
    Card(
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Text(
                text = "Hook 状态",
                style = MaterialTheme.typography.titleLarge
            )
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(if (status.isActive) "已激活" else "未激活")
                Switch(
                    checked = status.isActive,
                    onCheckedChange = { onToggle() }
                )
            }
            
            Text("过滤次数: ${status.filterCount}")
        }
    }
}
