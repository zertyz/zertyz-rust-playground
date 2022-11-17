# Demonstrates how to build a DLL and run it in a Metatrader 5 Expert Advisor.

# Development cycle

## new binary released to MT5 staging:
RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-pc-windows-gnu --release -p rust-mt5-bridge && ls -l ./target/x86_64-pc-windows-gnu/release/ && echo NOT DOING cp -av rust-mt5-bridge/RustMT5Bridge/* /mnt/nfs/data/tmp/RustMT5Bridge.staging/ && cp -av target/x86_64-pc-windows-gnu/release/rust_mt5_bridge.dll /mnt/nfs/data/tmp/RustMT5Bridge.staging/

## diffing before incorporating changes (from remote to local or vice-versa)
diff -Naur --strip-trailing-cr /mnt/nfs/data/tmp/RustMT5Bridge.staging/RustMt5Bridge.mq5 rust-mt5-bridge/RustMT5Bridge/RustMt5Bridge.mq5 | vim -

# Maintaining a backup alongside the staging
tar -cvJf /mnt/nfs/data/tmp/RustMT5Bridge.staging/source.backup.tar.xz rust-mt5-bridge/

# Having a look at the local logs:
cat rust_mt5_bridge.log ; rm rust_mt5_bridge.*