package com.hma.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.hma.data.ScopeConfig

@Composable
fun ConfigScreen(
    scopes: Map<String, ScopeConfig>,
    onAddScope: (String) -> Unit,
    onRemoveScope: (String) -> Unit,
    onEditScope: (String, ScopeConfig) -> Unit,
    onBack: () -> Unit
) {
    var showAddDialog by remember { mutableStateOf(false) }
    
    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("配置管理") },
            actions = {
                IconButton(onClick = { showAddDialog = true }) {
                    Text("+")
                }
            }
        )
        
        LazyColumn(modifier = Modifier.padding(16.dp)) {
            items(scopes.entries.toList()) { (pkg, config) ->
                ScopeItem(pkg, config, onEditScope, onRemoveScope)
            }
        }
    }
    
    if (showAddDialog) {
        AddScopeDialog(
            onDismiss = { showAddDialog = false },
            onConfirm = { pkg ->
                onAddScope(pkg)
                showAddDialog = false
            }
        )
    }
}

@Composable
fun ScopeItem(
    packageName: String,
    config: ScopeConfig,
    onEdit: (String, ScopeConfig) -> Unit,
    onRemove: (String) -> Unit
) {
    var expanded by remember { mutableStateOf(false) }
    
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp)
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(packageName, style = MaterialTheme.typography.titleMedium)
                IconButton(onClick = { onRemove(packageName) }) {
                    Text("×")
                }
            }
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text("模式: ${if (config.useWhitelist) "白名单" else "黑名单"}")
                Switch(
                    checked = config.useWhitelist,
                    onCheckedChange = { 
                        onEdit(packageName, config.copy(useWhitelist = it))
                    }
                )
            }
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text("排除系统应用")
                Switch(
                    checked = config.excludeSystemApps,
                    onCheckedChange = { 
                        onEdit(packageName, config.copy(excludeSystemApps = it))
                    }
                )
            }
            
            Text("隐藏应用: ${config.extraAppList.size} 个")
            Text("应用模板: ${config.applyTemplates.size} 个")
        }
    }
}

@Composable
fun AddScopeDialog(
    onDismiss: () -> Unit,
    onConfirm: (String) -> Unit
) {
    var packageName by remember { mutableStateOf("") }
    
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("添加 Scope") },
        text = {
            TextField(
                value = packageName,
                onValueChange = { packageName = it },
                label = { Text("包名") },
                placeholder = { Text("com.example.app") }
            )
        },
        confirmButton = {
            Button(
                onClick = { onConfirm(packageName) },
                enabled = packageName.isNotBlank()
            ) {
                Text("确定")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("取消")
            }
        }
    )
}
