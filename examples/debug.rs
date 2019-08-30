// Example for `debug_message` WasmVM function.
use xpx_supercontracts_sdk::statuses::STATUS_SUCCESS;
use xpx_supercontracts_sdk::utils::debug_message;

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    let msg = "Debug message".to_string();
    debug_message(msg);
    STATUS_SUCCESS
}
