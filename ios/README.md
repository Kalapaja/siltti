# siltti_ios

## GIT LFS
Compiled SilttiUnifiFFI.xcframework is tracked by GIT LFS
Install `git-lfs` tool and get it with `git-lfs fetch` command

## Rust
1. Add this to `uniffi.toml`:
```[bindings.swift]
module_name = "SilttiUniffi"
cdylib_name = "siltti"```
2. Put ios_build.sh in rust folder of siltti project
3. run `sh ios_build.sh` from it
4. It will create `SilttiUniffiFFI.xcframework` and `SilttiUniffi.swift`, add them into iOS project

## iOS
1. Run project on a real device supporting iOS 16+. Allow camera permission
2. On first launch, open Manage Networks and tap to add defaults (up to 3 networks)
3. From main screen scan QR code with payload or send blank payload
4. Use `Status/Read/Write` buttons for getting `NDEF Status/NDEF Message/Write APDU with payload` in endless cycle (until error is triggered)

## Links
- https://metadata.parity.io/#/polkadot
- https://polkadot.js.org/apps/#/accounts
