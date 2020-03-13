![Documentation](https://docs.rs/xpx-supercontracts-sdk/badge.svg)
# ProximaX Supercontracts Rust SDK

Official ProximaX Supercontracts SDK Library in Rust lang.

## Getting Started
All Supercontracts stuff include in dependency:
```toml
[dependencies]
xpx_supercontracts_sdk = "0.1"
```

To start new development new Supercontrac follow this steps:
1. `cargo new sc-app`

2. Add to `Cargo.toml`:
```toml
[dependencies]
xpx_supercontracts_sdk = "0.1"
```

3. Add to `src/main.rs`:
```rust
extern xpx_supercontracts_sdk;
use xpx_supercontracts_sdk::utils::ping;

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    ping(100).unwrap()
}
```

4. Build: `cargo build --target wasm32-unknown-unknown --release`
5. If build success result contains in: `target`
5. Convert to Wat/Wast format: `wasm2wat sc-app` 

## SDK Documentation
See `Docs` directory.

## Exampels
* See `eamples` directory
* Build specific exmaple: `cargo build --target wasm32-unknown-unknown --example ping`

### License: Apache 2.0
Copyright (c) 2019 ProximaX Limited
