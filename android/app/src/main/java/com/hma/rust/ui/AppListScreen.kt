package com.hma.rust.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.hma.rust.viewmodel.MainViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AppListScreen(
    viewModel: MainViewModel
) {
    var searchText by remember { mutableStateOf("") }
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // 搜索框
        OutlinedTextField(
            value = searchText,
            onValueChange = { searchText = it },
            label = { Text("搜索应用") },
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        )
        
        // 应用列表
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            val filteredApps = viewModel.apps.filter {
                it.appName.contains(searchText, ignoreCase = true) ||
                it.packageName.contains(searchText, ignoreCase = true)
            }
            
            items(filteredApps) { app ->
                Card(
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(16.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Column(
                            modifier = Modifier.weight(1f)
                        ) {
                            Text(
                                text = app.appName,
                                style = MaterialTheme.typography.bodyLarge
                            )
                            Text(
                                text = app.packageName,
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant
                            )
                        }
                        
                        Switch(
                            checked = app.isHidden,
                            onCheckedChange = { isHidden ->
                                if (isHidden) {
                                    viewModel.hideApp(app.packageName)
                                } else {
                                    viewModel.showApp(app.packageName)
                                }
                            }
                        )
                    }
                }
            }
        }
    }
}