//! Basic Blockchain functions and getters

use crate::external;
use crate::statuses::{Error, FunctionResult, Result};
use crate::transactions_type::{
	MosaicDefinition,
	MosaicDefinitionTransaction,
};

pub fn flush() -> FunctionResult {
	return unsafe { Ok(external::flush()) };
}

pub fn mosaic_definition(params: &MosaicDefinition) -> Result<MosaicDefinitionTransaction> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::mosaic_definition(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}
