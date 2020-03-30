# ProximaX Supercontracts Rust SDK examples

That directory provide basic examples for **How-To** work with SDK.

### Examples list
* `ping`
* `http`
* `debug`

## How to use
Run command `cargo build --target wasm32-unknown-unknown --example <example-name>`

Ex: `cargo build --target wasm32-unknown-unknown --example ping`

### Convert to wat file

For should be installed: https://github.com/WebAssembly/wabt

After build `wasm` for converting to `wat` run: `wasm2wat target/wasm32-unknown-unknown/release/examples/ping.wasm -o ~/ping.wat`
