package fi.zymologia.siltti.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Delete
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.uniffi.SpecsDisplay
import fi.zymologia.siltti.uniffi.SpecsKey


@Composable
fun NetworkCard(
    networks: MutableState<SpecsDisplay>,
    key: SpecsKey
) {
    Surface(
        color = MaterialTheme.colors.primary,
        modifier = Modifier
            .fillMaxWidth()
            .padding(10.dp)
    ) {
        Row(
            horizontalArrangement = Arrangement.SpaceBetween,
            modifier = Modifier.padding(10.dp)
        ) {
            Text(
                networks.value.title(key) ?: "unknown",
                color = MaterialTheme.colors.onPrimary
            )
            Text("Version: " + (networks.value.version(key) ?: "metadata unknown"))
        }
    }
}
