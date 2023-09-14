package fi.zymologia.siltti.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.Signer
import fi.zymologia.siltti.uniffi.Action

@Composable
fun NewAddress(
    setAppState: (Mode) -> Unit,
    transmitCallback: (Action?) -> Unit,
    dbName: String,
) {
    var address by remember { mutableStateOf("") }
    var hasPwd by remember { mutableStateOf(false) }

    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text("Type derivation")
        TextField(
            value = address,
            onValueChange = { address = it },
        )

        Row(
            horizontalArrangement = Arrangement.SpaceEvenly,
            modifier = Modifier.fillMaxWidth(1f),
        ) {
            Button(
                onClick = {
                    transmitCallback(Action.newDerivation(address, hasPwd, Signer()))
                    setAppState(Mode.TX)
                },
            ) {
                Text("Send")
            }
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
