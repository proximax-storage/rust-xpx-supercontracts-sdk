//! Basic utils functions for communication with `WasmVM`.

use crate::external;

/// Send ping message to `WasmVM`. Successful result should be
/// incremented value.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::utils::ping;
/// let respond = ping(10);
/// assert_eq!(respond, 11);
/// ```
pub fn ping(msg: usize) -> i64 {
    return unsafe { external::__ping(msg) };
}

/// Send debug message to `WasmVM`. It's convenient
/// basic function for development debugging.
/// Message that was sent will display in `WasmVM`
/// stdout as information log message. It not affect
/// basic Supercontract execution but should be
/// removed from `release` version, because it
/// will spend `Gas` (unit ticks).
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::utils::debug_message;
/// debug_message("Debug message from Supercontract".to_string());
/// ```
pub fn debug_message(msg: String) {
    let raw_msg = msg.as_bytes();
    unsafe {
        external::__write_log(raw_msg.as_ptr(), raw_msg.len());
    };
}