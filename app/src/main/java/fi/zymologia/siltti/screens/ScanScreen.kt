package fi.zymologia.siltti.screens

import android.annotation.SuppressLint
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.util.Log
import android.widget.Toast
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.ImageProxy
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.common.InputImage
import fi.zymologia.siltti.*
import fi.zymologia.siltti.components.ScanProgressBar
import fi.zymologia.siltti.uniffi.*
import fi.zymologia.siltti.uniffi.Action.Companion.newKampelaStop
import kotlin.reflect.KFunction1

@Composable
fun KeepScreenOn() {
    val currentView = LocalView.current
    DisposableEffect(Unit) {
        currentView.keepScreenOn = true
        onDispose {
            currentView.keepScreenOn = false
        }
    }
}

@Composable
fun ScanScreen(
    dbName: String,
    transmitCallback: (Action?) -> Unit,
    setAppState: (Mode) -> Unit,
) {
    val collection = remember { Collection() }
    val frames: MutableState<Frames?> = remember { mutableStateOf(null) }
    val lifecycleOwner = LocalLifecycleOwner.current
    val context = LocalContext.current
    val cameraProviderFuture =
        remember { ProcessCameraProvider.getInstance(context) }

    val error = remember { mutableStateOf("") }

    if (frames.value != null) {
        KeepScreenOn()
    }

    LazyColumn {
        item {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                AndroidView(
                    factory = { context ->
                        val executor = ContextCompat.getMainExecutor(context)
                        val previewView = PreviewView(context)
                        // mlkit docs: The default option is not recommended because it tries
                        // to scan all barcode formats, which is slow.
                        //
                        // In fact, it is not slow at all, no measurable difference was
                        // observed, but this avoids extra barcodes being accidentally scanned
                        // during multiframes.
                        val options =
                            BarcodeScannerOptions.Builder()
                                .setBarcodeFormats(Barcode.FORMAT_QR_CODE).build()

                        val barcodeScanner = BarcodeScanning.getClient(options)

                        // This might be done more elegantly, if needed.
                        // But it's pretty obvious that the app needs camera
                        // and why; also it just works so far and code is tiny.
                        handleCameraPermissions(context.getActivity())

                        cameraProviderFuture.addListener({
                            val cameraProvider = cameraProviderFuture.get()

                            val preview =
                                Preview.Builder().build().also {
                                    it.setSurfaceProvider(previewView.surfaceProvider)
                                }

                            val cameraSelector =
                                CameraSelector.Builder()
                                    .requireLensFacing(CameraSelector.LENS_FACING_BACK)
                                    .build()

                            val imageAnalysis =
                                ImageAnalysis.Builder()
                                    .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                                    .build()
                                    .apply {
                                        setAnalyzer(executor) { imageProxy ->
                                            processFrameWrapper(
                                                context,
                                                dbName,
                                                barcodeScanner,
                                                imageProxy,
                                                { transmittable: Action ->
                                                    transmitCallback(
                                                        transmittable,
                                                    )

                                                    if (!transmittable.isTransmit()) {
                                                        Toast
                                                            .makeText(
                                                                context,
                                                                "payload accepted",
                                                                Toast.LENGTH_SHORT,
                                                            ).show()
                                                    } else {
                                                        // Thanks to very smart electrical engineers in certain
                                                        // smartphone companies,
                                                        // NFC stops working when camera is on sometimes.
                                                        // This is too funny to be true but here we are.
                                                        cameraProvider.unbindAll()
                                                        setAppState(Mode.TX)
                                                    }
                                                },
                                                collection::processFrame,
                                                {
                                                    try {
                                                        frames.value = collection.frames()
                                                    } catch (e: ErrorQr) {
                                                        error.value = "QR scanner error: " + e.message
                                                    }
                                                },
                                                collection::clean,
                                            ) { error.value = it }
                                        }
                                    }

                            cameraProvider.unbindAll()
                            cameraProvider.bindToLifecycle(
                                lifecycleOwner,
                                cameraSelector,
                                imageAnalysis,
                                preview,
                            )
                        }, executor)
                        previewView
                    },
                    Modifier
                        .padding(bottom = 24.dp)
                        .border(
                            BorderStroke(1.dp, MaterialTheme.colors.primary),
                            RoundedCornerShape(8.dp),
                        )
                        .clip(RoundedCornerShape(8.dp)),
                )

                Column(
                    verticalArrangement = Arrangement.Bottom,
                    modifier = Modifier.fillMaxSize(),
                ) {
                    ScanProgressBar(
                        frames = frames,
                        resetScan = {
                            try {
                                collection.clean()
                                frames.value = collection.frames()
                            } catch (e: ErrorQr) {
                                error.value = "QR scanner reset error: " + e.message
                            }
                        },
                    )
                    Button(
                        onClick = {
                            transmitCallback(null)
                            cameraProviderFuture.get().unbindAll()
                            setAppState(Mode.Address)
                        },
                    ) {
                        Text("Create an address")
                    }
                    if (error.value.isNotBlank()) {
                        Text(error.value)
                        Button(
                            onClick = { error.value = "" },
                        ) {
                            Text("dismiss error")
                        }
                    }

                }
            }
        }
        item {
            Button(
                onClick = {
                    cameraProviderFuture.get().unbindAll()
                    setAppState(Mode.Networks)
                },
            ) {
                Text("ManageNetworks")
            }
        }
        item {
            Button(
                onClick = {
                    transmitCallback(newKampelaStop(Signer()))
                    cameraProviderFuture.get().unbindAll()
                    setAppState(Mode.TX)
                },
            ) {
                Text("Send blank payloads")
            }
        }
    }
}

/**
 * Barcode detecting function.
 * This uses experimental features
 */
@OptIn(ExperimentalUnsignedTypes::class)
@SuppressLint("UnsafeOptInUsageError")
fun processFrameWrapper(
    context: Context,
    dbName: String,
    barcodeScanner: BarcodeScanner,
    imageProxy: ImageProxy,
    startTransmission: (Action) -> Unit,
    submitFrame: KFunction1<ByteArray, Payload>,
    refreshFrames: () -> Unit,
    clean: () -> Unit,
    setError: (String) -> Unit,
) {
    if (imageProxy.image == null) return
    val inputImage =
        InputImage.fromMediaImage(
            imageProxy.image!!,
            imageProxy.imageInfo.rotationDegrees,
        )

    barcodeScanner.process(inputImage)
        .addOnSuccessListener { barcodes ->
            barcodes.forEach {
                it?.rawBytes?.let { payload ->
                    try {
                        submitFrame(payload)
                    } catch (e: ErrorQr) {
                        setError("QR parser error: " + e.message)
                        clean()
                        null
                    }
                }?.payload?.let { payload ->
                    // This is pressed only once, that's checked in rust backend
                    // by sending complete payload only once
                    try {
                        clean()
                        Log.e("payload content", payload.toString())
                        val action = Action.newPayload(payload, dbName, Signer())
                        startTransmission(action)
                    } catch (e: ErrorCompanion) {
                        setError("Payload parsing error: " + e.message)
                    }
                }
                refreshFrames()
            }
        }
        .addOnFailureListener {
            Log.e("Scan failed", it.message.toString())
        }
        .addOnCompleteListener {
            imageProxy.close()
        }
}

fun allPermissionsGranted(activity: Activity) =
    REQUIRED_PERMISSIONS.all {
        ContextCompat.checkSelfPermission(
            activity,
            it,
        ) == PackageManager.PERMISSION_GRANTED
    }
