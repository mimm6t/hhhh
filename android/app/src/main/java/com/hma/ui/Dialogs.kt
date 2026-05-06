package com.hma.ui

import android.content.Intent
import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext

@Composable
fun FilePickerDialog(
    title: String,
    onFilePicked: (Uri) -> Unit,
    onDismiss: () -> Unit
) {
    val launcher = rememberLauncherForActivityResult(
        ActivityResultContracts.GetContent()
    ) { uri ->
        uri?.let { onFilePicked(it) }
        onDismiss()
    }
    
    LaunchedEffect(Unit) {
        launcher.launch("application/json")
    }
}

@Composable
fun FileSaveDialog(
    title: String,
    defaultName: String,
    onFileSaved: (Uri) -> Unit,
    onDismiss: () -> Unit
) {
    val launcher = rememberLauncherForActivityResult(
        ActivityResultContracts.CreateDocument("application/json")
    ) { uri ->
        uri?.let { onFileSaved(it) }
        onDismiss()
    }
    
    LaunchedEffect(Unit) {
        launcher.launch(defaultName)
    }
}

@Composable
fun UpdateDialog(
    updateInfo: com.hma.data.UpdateInfo,
    onUpdate: () -> Unit,
    onDismiss: () -> Unit
) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("发现新版本") },
        text = {
            androidx.compose.foundation.layout.Column {
                Text("版本: ${updateInfo.version}")
                androidx.compose.foundation.layout.Spacer(modifier = androidx.compose.ui.Modifier.height(8.dp))
                Text("更新内容:")
                Text(updateInfo.changelog, style = MaterialTheme.typography.bodySmall)
            }
        },
        confirmButton = {
            Button(onClick = onUpdate) {
                Text("立即更新")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("稍后")
            }
        }
    )
}

@Composable
fun DownloadProgressDialog(
    progress: Int,
    onCancel: () -> Unit
) {
    AlertDialog(
        onDismissRequest = { },
        title = { Text("下载中...") },
        text = {
            androidx.compose.foundation.layout.Column {
                LinearProgressIndicator(
                    progress = progress / 100f,
                    modifier = androidx.compose.ui.Modifier.fillMaxWidth()
                )
                androidx.compose.foundation.layout.Spacer(modifier = androidx.compose.ui.Modifier.height(8.dp))
                Text("$progress%")
            }
        },
        confirmButton = {
            TextButton(onClick = onCancel) {
                Text("取消")
            }
        }
    )
}
