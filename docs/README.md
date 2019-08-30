# Rust SDK Documentation

## Functions
### utils::ping
* description: Send ping message to `WasmVM`. Successful 
result should be incremented value. Useful for most 
simple request/response message tests for  `WasmVM`.
* params: `msg: usize`
* return: `Result<i64>`

#### Example
```rust
use xpx_supercontracts_sdk::utils::ping;

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    ping(100).unwrap()
}

```

### utils::debug_message
* description: Send debug message to `WasmVM`. It's 
convenient basic function for development debugging. 
Message that was sent will display in `WasmVM` stdout 
as information log message. It not affect basic 
Supercontract execution but should be removed 
from `release` version, because it will spend `Gas` 
(unit ticks).
* params: `msg: &String`
* return: `()`

#### Example
```rust 
use xpx_supercontracts_sdk::statuses::STATUS_SUCCESS;
use xpx_supercontracts_sdk::utils::debug_message;

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    let msg = "Debug message".to_string();
    debug_message(msg);
    STATUS_SUCCESS
}
```
