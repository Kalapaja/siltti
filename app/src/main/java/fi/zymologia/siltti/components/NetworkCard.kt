package fi.zymologia.siltti.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.uniffi.Key
import fi.zymologia.siltti.uniffi.Selector

@Composable
fun NetworkCard(
    networks: MutableState<Selector>,
    key: Key,
) {
    Surface(
        color = MaterialTheme.colors.primary,
        modifier =
            Modifier
                .fillMaxWidth()
                .padding(10.dp),
    ) {
        Row(
            horizontalArrangement = Arrangement.SpaceBetween,
            modifier = Modifier.padding(10.dp),
        ) {
            Text(
                networks.value.name(key) ?: "unknown",
                color = MaterialTheme.colors.onPrimary,
            )
            Text("Version: " + (networks.value.version(key) ?: "metadata unknown"))
        }
    }
}
