# Demonstrates how to build a DLL and run it in a Metatrader 5 Expert Advisor.

# Development cycle

## new binary released to MT5 staging:
build_type="debug"; build_flag="--${build_type}"; RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-pc-windows-gnu ${build_flag/--debug/} -p rust-mt5-bridge && ls -l /mnt/nfs/data/tmp/RustMT5Bridge.staging/rust_mt5_bridge.dll target/x86_64-pc-windows-gnu/${build_type}/rust_mt5_bridge.dll && cp target/x86_64-pc-windows-gnu/${build_type}/rust_mt5_bridge.dll /mnt/nfs/data/tmp/RustMT5Bridge.staging/ && ls -l /mnt/nfs/data/tmp/RustMT5Bridge.staging/rust_mt5_bridge.dll

## diffing before incorporating changes (from remote to local or vice-versa)
for file in JAson.mqh RustToMQLMethodCall.mqh AccountInfoBridge.mqh DealPropertiesBridge.mqh RustDll.mqh RustMt5Bridge.mq5 SymbolInfoBridge.mqh TesterScript.mq5 EnumReporter.mqh; do SOURCE="/mnt/nfs/data/tmp/RustMT5Bridge.staging/$file"; TARGET="rust-mt5-bridge/RustMT5Bridge/$file"; diff -Naur --strip-trailing-cr "$SOURCE" "$TARGET" | vim -; echo; echo -en "Type COPY to copy '$SOURCE' to '$TARGET'... ENTER to abort"; read input; if [ "$input" == "COPY" ]; then cp -v "$SOURCE" "$TARGET"; fi; done

# Maintaining a backup alongside the staging
tar -cvJf /mnt/nfs/data/tmp/RustMT5Bridge.staging/source.backup.tar.xz rust-mt5-bridge/

# Deploying from staging
cp -av /mnt/nfs/data/tmp/RustMT5Bridge.staging/* /mnt/nfs/data/tmp/RustMT5Bridge/

# Having a look at the local logs:
cat rust_mt5_bridge.log ; rm rust_mt5_bridge.*
