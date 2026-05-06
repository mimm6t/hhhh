package com.hma.rust.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.hma.rust.viewmodel.MainViewModel

@Composable
fun MainScreen(
    viewModel: MainViewModel,
    onNavigateToApps: () -> Unit = {},
    onNavigateToConfig: () -> Unit = {},
    onNavigateToTemplates: () -> Unit = {},
    onNavigateToLogs: () -> Unit = {}
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // 状态卡片
        Card(
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = "Hook 状态",
                    style = MaterialTheme.typography.headlineSmall
                )
                Spacer(modifier = Modifier.height(8.dp))
                Row(
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = if (viewModel.isHookActive) "已激活" else "未激活",
                        modifier = Modifier.weight(1f)
                    )
                    Switch(
                        checked = viewModel.isHookActive,
                        onCheckedChange = { viewModel.toggleHook() }
                    )
                }
            }
        }
        
        // 快捷操作
        Text(
            text = "快捷操作",
            style = MaterialTheme.typography.headlineSmall
        )
        
        Button(
            onClick = onNavigateToApps,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("管理应用")
        }
        
        Button(
            onClick = onNavigateToConfig,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("作用域配置")
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
            Text("查看日志")
        }
    }
}