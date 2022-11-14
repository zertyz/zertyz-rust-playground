Demonstrates how to build a DLL and run it in a Metatrader 5 Expert Advisor.

Example setup pipeline:
cargo build --target x86_64-pc-windows-gnu --release -p rust-mt5-bridge && ls -l ./target/x86_64-pc-windows-gnu/release/ && cp -av rust-mt5-bridge/RustMT5Bridge /mnt/nfs/data/tmp/ && cp -av target/x86_64-pc-windows-gnu/release/rust_mt5_bridge.dll /mnt/nfs/data/tmp/RustMT5Bridge/

Development -- new binary released to MT5:
cargo build --target x86_64-pc-windows-gnu --release -p rust-mt5-bridge && ls -l ./target/x86_64-pc-windows-gnu/release/ && echo NOT DOING cp -av rust-mt5-bridge/RustMT5Bridge /mnt/nfs/data/tmp/ && cp -av target/x86_64-pc-windows-gnu/release/rust_mt5_bridge.dll /mnt/nfs/data/tmp/RustMT5Bridge/

Sources from MT5 reimported here:
cp -a /mnt/nfs/data/tmp/RustMT5Bridge/RustMt5Bridge.mq5 rust-mt5-bridge/RustMT5Bridge/RustMt5Bridge.mq5

Having a look at the local logs:
cat rust_mt5_bridge.log ; rm rust_mt5_bridge.*