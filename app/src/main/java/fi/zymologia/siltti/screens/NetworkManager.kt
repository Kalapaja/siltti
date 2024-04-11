package fi.zymologia.siltti.screens

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.components.NetworkCard
import fi.zymologia.siltti.uniffi.Key.Companion.import
import fi.zymologia.siltti.uniffi.Selector

@Composable
fun NetworkManager(
    dbName: String,
    setAppState: (Mode) -> Unit,
) {
    val networks = remember { mutableStateOf(Selector(dbName)) }

    val rpcServer = remember { mutableStateOf("") }

    LazyColumn {
        item {
            Button(
                onClick = {
                    networks.value.setupDefaults(dbName)
                },
            ) {
                Text("Add defaults!")
            }
        }
        item{
            Text("Available networks", style = MaterialTheme.typography.h4)
        }
        this.items(
            items = networks.value.getAllKeys().map { it.export() } ,
            key = { it },
        ) { key ->
            NetworkCard(networks, import(key))
        }
        item {
            TextField(value = rpcServer.value, onValueChange = { rpcServer.value = it })
        }
        item {
            Button(
                onClick = {
                    networks.value.addNewElement(rpcServer.value, dbName)
                },
            ) {
                Text("Add new network")
            }
        }
        item {
            Button(
                onClick = {
                    setAppState(Mode.TX)
                },
            ) {
                Text("Back to scan")
            }
        }
    }
}
