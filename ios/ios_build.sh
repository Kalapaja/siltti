#!/bin/bash

# 1. Build the static library for iOS arm64 architecture
cargo build --release --target aarch64-apple-ios

# 2. Generate Swift bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-ios/release/libsiltti.a --language swift --out-dir ./bindings

# 3. Move necessary files for the XCFramework creation
mv bindings/SilttiUniffiFFI.modulemap bindings/module.modulemap

# 4. Create the XCFramework using the static library
xcodebuild -create-xcframework \
    -library ./target/aarch64-apple-ios/release/libsiltti.a \
    -headers ./bindings \
    -output "ios/SilttiUniffiFFI.xcframework"

# 5. Move the generated Swift file for proper use in your iOS project
mv bindings/SilttiUniffi.swift ios/SilttiUniffi.swift

# 6. Clean up the temporary bindings folder
rm -rf bindings
