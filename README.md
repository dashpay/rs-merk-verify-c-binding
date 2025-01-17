# rs-merk-verify-c-binding
Merk verification for C and iOS/MacOS

###### Prerequisites:
```
cargo install cargo-lipo
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
```

###### Create universal binary (iOS): 
```
cargo lipo --release
```

###### Create MacOS version:
```
cargo build --target=x86_64-apple-darwin --release
cargo build --target=aarch64-apple-darwin --release
lipo -create target/aarch64-apple-darwin/release/libmerk_ios.a target/x86_64-apple-darwin/release/libmerk_ios.a -output target/universal/release/libmerkMacOS.a
```

Use from Obj-C with NSData:
Add generated merk.h

```obj-c    
NSData *proofData = ...
// proof data
ExecuteProofResult *result = execute_proof_c(proofData.bytes, proofData.length);
// cleanup memory
destroy_proof_c(result);
```
