// Example for `ping` WasmVM function.
use xpx_supercontracts_sdk::utils::ping;

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    ping(100).unwrap()
}
