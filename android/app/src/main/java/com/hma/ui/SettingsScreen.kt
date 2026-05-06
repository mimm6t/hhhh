package com.hma.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun SettingsScreen(
    detailLog: Boolean,
    maxLogSize: Int,
    onDetailLogChange: (Boolean) -> Unit,
    onMaxLogSizeChange: (Int) -> Unit,
    onExportConfig: () -> Unit,
    onImportConfig: () -> Unit,
    onCheckUpdate: () -> Unit,
    onAbout: () -> Unit,
    onBack: () -> Unit
) {
    LazyColumn(
        modifier = Modifier.fillMaxSize()
    ) {
        item {
            TopAppBar(title = { Text("设置") })
        }
        
        item {
            SettingsSection(title = "日志设置") {
                SettingsSwitch(
                    title = "详细日志",
                    subtitle = "记录更多调试信息",
                    checked = detailLog,
                    onCheckedChange = onDetailLogChange
                )
                
                SettingsSlider(
                    title = "最大日志大小",
                    subtitle = "$maxLogSize KB",
                    value = maxLogSize.toFloat(),
                    onValueChange = { onMaxLogSizeChange(it.toInt()) },
                    valueRange = 512f..4096f
                )
            }
        }
        
        item {
            SettingsSection(title = "配置管理") {
                SettingsItem(
                    title = "导出配置",
                    subtitle = "保存当前配置到文件",
                    onClick = onExportConfig
                )
                
                SettingsItem(
                    title = "导入配置",
                    subtitle = "从文件加载配置",
                    onClick = onImportConfig
                )
            }
        }
        
        item {
            SettingsSection(title = "关于") {
                SettingsItem(
                    title = "检查更新",
                    subtitle = "查看是否有新版本",
                    onClick = onCheckUpdate
                )
                
                SettingsItem(
                    title = "关于应用",
                    subtitle = "版本 0.2.0",
                    onClick = onAbout
                )
            }
        }
    }
}

@Composable
fun SettingsSection(
    title: String,
    content: @Composable () -> Unit
) {
    Column(modifier = Modifier.padding(vertical = 8.dp)) {
        Text(
            text = title,
            style = MaterialTheme.typography.titleSmall,
            color = MaterialTheme.colorScheme.primary,
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
        )
        content()
        Divider()
    }
}

@Composable
fun SettingsItem(
    title: String,
    subtitle: String,
    onClick: () -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
            .padding(16.dp)
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
            Text(
                text = subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

@Composable
fun SettingsSwitch(
    title: String,
    subtitle: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
            Text(
                text = subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange
        )
    }
}

@Composable
fun SettingsSlider(
    title: String,
    subtitle: String,
    value: Float,
    onValueChange: (Float) -> Unit,
    valueRange: ClosedFloatingPointRange<Float>
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp)
    ) {
        Text(text = title, style = MaterialTheme.typography.bodyLarge)
        Text(
            text = subtitle,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Slider(
            value = value,
            onValueChange = onValueChange,
            valueRange = valueRange,
            modifier = Modifier.fillMaxWidth()
        )
    }
}

@Composable
fun AboutDialog(onDismiss: () -> Unit) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("关于 Hide My Applist") },
        text = {
            Column {
                Text("版本: 0.2.0")
                Spacer(modifier = Modifier.height(8.dp))
                Text("基于 wxshadow 内核模块的应用列表隐藏工具")
                Spacer(modifier = Modifier.height(8.dp))
                Text("技术栈:")
                Text("• Rust 核心库")
                Text("• Jetpack Compose UI")
                Text("• W^X Shadow 技术")
            }
        },
        confirmButton = {
            Button(onClick = onDismiss) {
                Text("确定")
            }
        }
    )
}
