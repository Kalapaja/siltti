# siltti
Companion app for Kampela hardware signer

This is a reference implementation for hot-side protocol of Kampela device. This app converts QR codes from Polkadot Vault ecosystem into appropriate Kampela protocol NFC payloads stream.

## Build requirements

1. Android Studio or XCode
2. uniffi

Note that for iOS build you'll need an actual developer account to be able to have any access to NFC chip at all.

## Android build notes

Remeber to bump app's NDK version to one provided by your version of Android Studio in File > Project Structure > Modules

## iOS implementation

Build with XCode
