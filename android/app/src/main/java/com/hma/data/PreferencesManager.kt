package com.hma.data

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.*
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private val Context.dataStore: DataStore<Preferences> by preferencesDataStore(name = "settings")

class PreferencesManager(private val context: Context) {
    private val DETAIL_LOG = booleanPreferencesKey("detail_log")
    private val MAX_LOG_SIZE = intPreferencesKey("max_log_size")
    private val LAST_UPDATE_CHECK = longPreferencesKey("last_update_check")
    private val AUTO_CHECK_UPDATE = booleanPreferencesKey("auto_check_update")
    
    val detailLog: Flow<Boolean> = context.dataStore.data.map { it[DETAIL_LOG] ?: false }
    val maxLogSize: Flow<Int> = context.dataStore.data.map { it[MAX_LOG_SIZE] ?: 1024 }
    val autoCheckUpdate: Flow<Boolean> = context.dataStore.data.map { it[AUTO_CHECK_UPDATE] ?: true }
    
    suspend fun setDetailLog(enabled: Boolean) {
        context.dataStore.edit { it[DETAIL_LOG] = enabled }
    }
    
    suspend fun setMaxLogSize(size: Int) {
        context.dataStore.edit { it[MAX_LOG_SIZE] = size }
    }
    
    suspend fun setLastUpdateCheck(time: Long) {
        context.dataStore.edit { it[LAST_UPDATE_CHECK] = time }
    }
    
    suspend fun setAutoCheckUpdate(enabled: Boolean) {
        context.dataStore.edit { it[AUTO_CHECK_UPDATE] = enabled }
    }
    
    suspend fun getLastUpdateCheck(): Long {
        return context.dataStore.data.map { it[LAST_UPDATE_CHECK] ?: 0L }.first()
    }
    
    private suspend fun <T> Flow<T>.first(): T {
        var result: T? = null
        collect { result = it }
        return result!!
    }
}
