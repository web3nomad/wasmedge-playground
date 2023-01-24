# WasmEdge playground


### Build

```shell
cargo build --target wasm32-wasi
# for release build
cargo build --target wasm32-wasi --release
# or with cargo-watch, automatically build on file changes
cargo watch -x "build --target wasm32-wasi"
```

### Run

```shell
wasmedge target/wasm32-wasi/debug/workflow.wasm second state
```
