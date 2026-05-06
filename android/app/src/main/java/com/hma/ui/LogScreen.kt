package com.hma.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import java.text.SimpleDateFormat
import java.util.*

data class LogEntry(
    val timestamp: Long,
    val level: LogLevel,
    val message: String
)

enum class LogLevel {
    DEBUG, INFO, WARN, ERROR
}

@Composable
fun LogScreen(
    logs: List<LogEntry>,
    onClearLogs: () -> Unit,
    onRefresh: () -> Unit,
    onBack: () -> Unit
) {
    var filterLevel by remember { mutableStateOf<LogLevel?>(null) }
    var searchQuery by remember { mutableStateOf("") }
    
    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("日志查看") },
            actions = {
                IconButton(onClick = onRefresh) {
                    Text("↻")
                }
                IconButton(onClick = onClearLogs) {
                    Text("🗑")
                }
            }
        )
        
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            FilterChip(
                selected = filterLevel == null,
                onClick = { filterLevel = null },
                label = { Text("全部") }
            )
            FilterChip(
                selected = filterLevel == LogLevel.ERROR,
                onClick = { filterLevel = LogLevel.ERROR },
                label = { Text("错误") }
            )
            FilterChip(
                selected = filterLevel == LogLevel.WARN,
                onClick = { filterLevel = LogLevel.WARN },
                label = { Text("警告") }
            )
            FilterChip(
                selected = filterLevel == LogLevel.INFO,
                onClick = { filterLevel = LogLevel.INFO },
                label = { Text("信息") }
            )
        }
        
        TextField(
            value = searchQuery,
            onValueChange = { searchQuery = it },
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 8.dp),
            placeholder = { Text("搜索日志...") }
        )
        
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(8.dp)
        ) {
            items(
                logs.filter { log ->
                    (filterLevel == null || log.level == filterLevel) &&
                    (searchQuery.isBlank() || log.message.contains(searchQuery, ignoreCase = true))
                }
            ) { log ->
                LogItem(log)
            }
        }
    }
}

@Composable
fun LogItem(log: LogEntry) {
    val dateFormat = remember { SimpleDateFormat("HH:mm:ss", Locale.getDefault()) }
    val color = when (log.level) {
        LogLevel.DEBUG -> Color.Gray
        LogLevel.INFO -> Color.Blue
        LogLevel.WARN -> Color(0xFFFFA500)
        LogLevel.ERROR -> Color.Red
    }
    
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 2.dp),
        colors = CardDefaults.cardColors(
            containerColor = color.copy(alpha = 0.1f)
        )
    ) {
        Column(modifier = Modifier.padding(8.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = log.level.name,
                    style = MaterialTheme.typography.labelSmall,
                    color = color
                )
                Text(
                    text = dateFormat.format(Date(log.timestamp)),
                    style = MaterialTheme.typography.labelSmall
                )
            }
            Text(
                text = log.message,
                style = MaterialTheme.typography.bodySmall
            )
        }
    }
}

// 日志管理器
object LogManager {
    private val _logs = mutableStateListOf<LogEntry>()
    val logs: List<LogEntry> get() = _logs
    
    fun addLog(level: LogLevel, message: String) {
        _logs.add(0, LogEntry(System.currentTimeMillis(), level, message))
        if (_logs.size > 1000) {
            _logs.removeAt(_logs.lastIndex)
        }
    }
    
    fun clear() {
        _logs.clear()
    }
    
    fun debug(message: String) = addLog(LogLevel.DEBUG, message)
    fun info(message: String) = addLog(LogLevel.INFO, message)
    fun warn(message: String) = addLog(LogLevel.WARN, message)
    fun error(message: String) = addLog(LogLevel.ERROR, message)
}
