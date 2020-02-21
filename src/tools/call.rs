//! Basic additional toolkit for SuperContract functions 
//!

use serde::{de::DeserializeOwned, Serialize};
use crate::statuses::{Error, Result};

/// External function type definition for call 
/// closure external FFI functions 
type ExternalFn = unsafe extern "C" fn(
	data_ptr: *const u8,
	data_len: usize,
	transaction: *mut u8,
) -> i64;

/// External functions call wrapper. It serialize input data
/// and deserialize output data after raw call of external function.
/// It support any kind ofr external functions to wrap for basic flow
/// for SuperContract functions with specific flow like this:
/// * serialize input data
/// * invoke external function with serialized function parameters
/// * fetch external function result
/// * deserialize external function to specific type for SuperContract function 
pub fn call_external_func<T, U>(params: &T, extenral_fn: ExternalFn) -> Result<U>
	where T: Serialize, U: DeserializeOwned {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = extenral_fn(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};

	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}
