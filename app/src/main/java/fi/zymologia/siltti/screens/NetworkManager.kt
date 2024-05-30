package fi.zymologia.siltti.screens

import android.annotation.SuppressLint
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.LiveData
import androidx.lifecycle.Observer
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.components.NetworkCard
import fi.zymologia.siltti.uniffi.ChainKey
import fi.zymologia.siltti.uniffi.getAllKeys
import fi.zymologia.siltti.uniffi.isUpdated
import fi.zymologia.siltti.uniffi.requestDefaults
import fi.zymologia.siltti.uniffi.requestFullFetch
import kotlinx.coroutines.delay

@SuppressLint("UnrememberedMutableState")
@Composable
fun NetworkManager(
    dbName: String,
    setAppState: (Mode) -> Unit,
) {
    val rpcServer = remember { mutableStateOf("") }

    val chainKeys = mutableStateOf(getAllKeys(dbName)) // Yes, I want it recomposed every time for now

    val updated = remember { mutableStateOf(false) }

    LaunchedEffect(updated.value) {
        while (!isUpdated(dbName)) {
            delay(1000)
        }
        chainKeys.value = getAllKeys(dbName)
        updated.value = !updated.value
    }
    LazyColumn {
        item {
            Button(
                onClick = {
                    requestDefaults();
                },
            ) {
                Text("Add defaults!")
            }
        }
        item{
            Text("Available networks", style = MaterialTheme.typography.h4)
        }
        this.items(
            items = chainKeys.value,
            key = { it },
        ) { key ->
            NetworkCard(key, dbName)
        }
        item {
            TextField(value = rpcServer.value, onValueChange = { rpcServer.value = it })
        }
        item {
            Button(
                onClick = {
                    requestFullFetch(rpcServer.value)
                },
            ) {
                Text("Add new network")
            }
        }
        item {
            Button(
                onClick = {
                    setAppState(Mode.Scan)
                },
            ) {
                Text("Back to scan")
            }
        }
    }
}
