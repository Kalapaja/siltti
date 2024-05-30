package fi.zymologia.siltti.components

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.uniffi.ChainKey
import fi.zymologia.siltti.uniffi.deleteByKey
import fi.zymologia.siltti.uniffi.requestUpdateByKey

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun NetworkCard(
    key: ChainKey,
    dbName: String,
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
                key.substring(0, 64),
                color = MaterialTheme.colors.onPrimary,
                modifier = Modifier.combinedClickable(onClick = { requestUpdateByKey(key, dbName) }, onLongClick = { deleteByKey(key, dbName) }),
            )
            // Text("Version: " + "PUT NETWORK VERSION HERE")
        }
    }
}
