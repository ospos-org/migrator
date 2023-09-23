cargo build --target wasm32-wasi --release
wasmtime target/wasm32-wasi/release/odm_migration_utility.wasm
cp target/wasm32-wasi/release/odm_migration_utility.wasm ./pkg/odm.wasm