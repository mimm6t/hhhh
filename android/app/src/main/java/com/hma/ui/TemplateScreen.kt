package com.hma.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.hma.data.Template

@Composable
fun TemplateScreen(
    templates: Map<String, Template>,
    onAddTemplate: (String, Set<String>) -> Unit,
    onRemoveTemplate: (String) -> Unit,
    onEditTemplate: (String, Template) -> Unit,
    onBack: () -> Unit
) {
    var showAddDialog by remember { mutableStateOf(false) }
    
    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("模板管理") },
            actions = {
                IconButton(onClick = { showAddDialog = true }) {
                    Text("+")
                }
            }
        )
        
        LazyColumn(modifier = Modifier.padding(16.dp)) {
            items(templates.entries.toList()) { (name, template) ->
                TemplateItem(name, template, onEditTemplate, onRemoveTemplate)
            }
        }
    }
    
    if (showAddDialog) {
        AddTemplateDialog(
            onDismiss = { showAddDialog = false },
            onConfirm = { name, apps ->
                onAddTemplate(name, apps)
                showAddDialog = false
            }
        )
    }
}

@Composable
fun TemplateItem(
    name: String,
    template: Template,
    onEdit: (String, Template) -> Unit,
    onRemove: (String) -> Unit
) {
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
                Text(name, style = MaterialTheme.typography.titleMedium)
                IconButton(onClick = { onRemove(name) }) {
                    Text("×")
                }
            }
            
            Text("包含应用: ${template.appList.size} 个")
            
            template.appList.take(3).forEach { pkg ->
                Text("  • $pkg", style = MaterialTheme.typography.bodySmall)
            }
            
            if (template.appList.size > 3) {
                Text("  ... 还有 ${template.appList.size - 3} 个", 
                    style = MaterialTheme.typography.bodySmall)
            }
        }
    }
}

@Composable
fun AddTemplateDialog(
    onDismiss: () -> Unit,
    onConfirm: (String, Set<String>) -> Unit
) {
    var templateName by remember { mutableStateOf("") }
    var appList by remember { mutableStateOf("") }
    
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("添加模板") },
        text = {
            Column {
                TextField(
                    value = templateName,
                    onValueChange = { templateName = it },
                    label = { Text("模板名称") },
                    modifier = Modifier.fillMaxWidth()
                )
                Spacer(modifier = Modifier.height(8.dp))
                TextField(
                    value = appList,
                    onValueChange = { appList = it },
                    label = { Text("应用列表（每行一个）") },
                    modifier = Modifier.fillMaxWidth(),
                    minLines = 3
                )
            }
        },
        confirmButton = {
            Button(
                onClick = {
                    val apps = appList.lines()
                        .map { it.trim() }
                        .filter { it.isNotBlank() }
                        .toSet()
                    onConfirm(templateName, apps)
                },
                enabled = templateName.isNotBlank() && appList.isNotBlank()
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

// 预设模板
object PresetTemplates {
    val ROOT_TOOLS = Template(
        name = "Root 工具",
        appList = setOf(
            "com.topjohnwu.magisk",
            "me.weishu.kernelsu",
            "me.bmax.apatch"
        )
    )
    
    val XPOSED_MODULES = Template(
        name = "Xposed 模块",
        appList = setOf(
            "de.robv.android.xposed.installer",
            "org.lsposed.manager",
            "icu.nullptr.hidemyapplist"
        )
    )
    
    val VIRTUAL_APPS = Template(
        name = "虚拟应用",
        appList = setOf(
            "com.lody.virtual",
            "io.va.exposed"
        )
    )
    
    fun getAll() = mapOf(
        "root_tools" to ROOT_TOOLS,
        "xposed_modules" to XPOSED_MODULES,
        "virtual_apps" to VIRTUAL_APPS
    )
}
