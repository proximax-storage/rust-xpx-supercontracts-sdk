//! Basic utils functions for communication with `WasmVM`.

use crate::external;
use crate::statuses::FunctionResult;

/// Init is function constructor that can can invoked only one time.
/// 
/// Most useful case is run some specifuc functionality and
/// functions to tune-up and prepare some state for SuperContract.
///
/// It's impossible run that function twice.
/// 
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::utils::{init, ping};
/// init(|| {
///     let respond = ping(10);
///     assert_eq!(respond.unwrap(), 11); 
/// });
/// ```
/// 
pub fn init(init_handler: fn() -> ()) {
	unsafe {
		let status = external::init();
		if status != 0 {
			return;
		}
	};
	init_handler();
}

/// Send ping message to `WasmVM`. Successful result should be
/// incremented value. Useful for most simple request/response
/// message tests for  `WasmVM`.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::utils::ping;
/// let respond = ping(10);
/// assert_eq!(respond.unwrap(), 11);
/// ```
pub fn ping(msg: usize) -> FunctionResult {
	return unsafe { Ok(external::__ping(msg)) };
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
/// debug_message(&"Debug message from Supercontract".to_string());
/// ```
pub fn debug_message(msg: &String) {
	let raw_msg = msg.as_bytes();
	unsafe {
		external::__write_log(raw_msg.as_ptr(), raw_msg.len());
	};
}
