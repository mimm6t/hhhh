package com.hma.data

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File

object FileManager {
    private const val CONFIG_FILE = "config.json"
    private const val BACKUP_DIR = "backups"
    
    suspend fun saveConfig(context: Context, config: Config): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val json = ConfigSerializer.toJson(config)
            context.openFileOutput(CONFIG_FILE, Context.MODE_PRIVATE).use {
                it.write(json.toByteArray())
            }
        }
    }
    
    suspend fun loadConfig(context: Context): Result<Config> = withContext(Dispatchers.IO) {
        runCatching {
            val json = context.openFileInput(CONFIG_FILE).bufferedReader().use { it.readText() }
            ConfigSerializer.fromJson(json)
        }
    }
    
    suspend fun exportConfig(context: Context, config: Config, path: String): Result<Unit> = withContext(Dispatchers.IO) {
        runCatching {
            val json = ConfigSerializer.toJson(config)
            File(path).writeText(json)
        }
    }
    
    suspend fun importConfig(context: Context, path: String): Result<Config> = withContext(Dispatchers.IO) {
        runCatching {
            val json = File(path).readText()
            ConfigSerializer.fromJson(json)
        }
    }
    
    suspend fun createBackup(context: Context, config: Config): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            val backupDir = File(context.filesDir, BACKUP_DIR)
            backupDir.mkdirs()
            
            val timestamp = System.currentTimeMillis()
            val backupFile = File(backupDir, "config_$timestamp.json")
            
            val json = ConfigSerializer.toJson(config)
            backupFile.writeText(json)
            
            backupFile.absolutePath
        }
    }
    
    suspend fun listBackups(context: Context): Result<List<File>> = withContext(Dispatchers.IO) {
        runCatching {
            val backupDir = File(context.filesDir, BACKUP_DIR)
            backupDir.listFiles()?.sortedByDescending { it.lastModified() }?.toList() ?: emptyList()
        }
    }
}

object ConfigSerializer {
    fun toJson(config: Config): String {
        val scopes = config.scope.entries.joinToString(",\n    ") { (pkg, cfg) ->
            """"$pkg": {
      "use_whitelist": ${cfg.useWhitelist},
      "exclude_system_apps": ${cfg.excludeSystemApps},
      "extra_app_list": [${cfg.extraAppList.joinToString(",") { "\"$it\"" }}],
      "apply_templates": [${cfg.applyTemplates.joinToString(",") { "\"$it\"" }}]
    }"""
        }
        
        val templates = config.templates.entries.joinToString(",\n    ") { (name, tpl) ->
            """"$name": {
      "name": "${tpl.name}",
      "app_list": [${tpl.appList.joinToString(",") { "\"$it\"" }}]
    }"""
        }
        
        return """{
  "config_version": ${config.configVersion},
  "detail_log": ${config.detailLog},
  "max_log_size": ${config.maxLogSize},
  "scope": {
    $scopes
  },
  "templates": {
    $templates
  }
}"""
    }
    
    fun fromJson(json: String): Config {
        // Simple JSON parsing (in production, use kotlinx.serialization or Gson)
        val config = Config()
        // TODO: Parse JSON properly
        return config
    }
}
