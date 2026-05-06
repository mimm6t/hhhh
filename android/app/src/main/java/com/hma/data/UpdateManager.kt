package com.hma.data

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.net.URL

data class UpdateInfo(
    val version: String,
    val versionCode: Int,
    val downloadUrl: String,
    val changelog: String,
    val isRequired: Boolean = false
)

object UpdateManager {
    private const val UPDATE_URL = "https://api.github.com/repos/example/hma-rust/releases/latest"
    private const val CURRENT_VERSION = "0.2.0"
    private const val CURRENT_VERSION_CODE = 2
    
    suspend fun checkUpdate(): Result<UpdateInfo?> = withContext(Dispatchers.IO) {
        runCatching {
            val response = URL(UPDATE_URL).readText()
            parseUpdateInfo(response)
        }
    }
    
    private fun parseUpdateInfo(json: String): UpdateInfo? {
        // Simple parsing (in production, use proper JSON parser)
        if (!json.contains("tag_name")) return null
        
        val version = extractValue(json, "tag_name")
        val downloadUrl = extractValue(json, "browser_download_url")
        val changelog = extractValue(json, "body")
        
        val versionCode = version.replace(".", "").toIntOrNull() ?: 0
        
        return if (versionCode > CURRENT_VERSION_CODE) {
            UpdateInfo(
                version = version,
                versionCode = versionCode,
                downloadUrl = downloadUrl,
                changelog = changelog
            )
        } else null
    }
    
    private fun extractValue(json: String, key: String): String {
        val pattern = """"$key":\s*"([^"]*)"""".toRegex()
        return pattern.find(json)?.groupValues?.get(1) ?: ""
    }
    
    suspend fun downloadUpdate(url: String, onProgress: (Int) -> Unit): Result<String> = withContext(Dispatchers.IO) {
        runCatching {
            val connection = URL(url).openConnection()
            val totalSize = connection.contentLength
            val outputFile = "/sdcard/Download/hma-update.apk"
            
            connection.getInputStream().use { input ->
                java.io.FileOutputStream(outputFile).use { output ->
                    val buffer = ByteArray(8192)
                    var downloaded = 0
                    var read: Int
                    
                    while (input.read(buffer).also { read = it } != -1) {
                        output.write(buffer, 0, read)
                        downloaded += read
                        onProgress((downloaded * 100 / totalSize))
                    }
                }
            }
            
            outputFile
        }
    }
}
