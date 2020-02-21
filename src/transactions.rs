//! Basic Blockchain functions and getters

use serde::{de::DeserializeOwned, Serialize};

use crate::external;
use crate::statuses::{Error, FunctionResult, Result};
use crate::transactions_type::{GetAccountExchangeInfo, GetExchangeOfferByAssetId, GetMosaicInfo, GetMosaicInfos, GetMosaicsNames, MosaicDefinition, MosaicDefinitionTransaction, MosaicInfo, MosaicName, OfferInfo, UserExchangeInfo};

/// Extrnral function type definition for call closure func 
type ExternalFn = unsafe extern "C" fn(
	data_ptr: *const u8,
	data_len: usize,
	transaction: *mut u8,
) -> i64;

/// External functions call wrapper. It serialize input data
/// and deserialize output data after raw call of external function
fn call_external_func<T, U>(params: &T, extenral_fn: ExternalFn) -> Result<U>
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

pub fn flush() -> FunctionResult {
	return unsafe { Ok(external::flush()) };
}

pub fn mosaic_definition(params: &MosaicDefinition) -> Result<MosaicDefinitionTransaction> {
	call_external_func(params, external::mosaic_definition)
}

pub fn get_account_exchange_info(params: &GetAccountExchangeInfo) -> Result<UserExchangeInfo> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_account_exchange_info(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}

pub fn get_exchange_offer_by_asset_id(params: &GetExchangeOfferByAssetId) -> Result<OfferInfo> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_exchange_offer_by_asset_id(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}

pub fn get_mosaic_info(params: &GetMosaicInfo) -> Result<Option<MosaicInfo>> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_mosaic_info(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}

pub fn get_mosaic_infos(params: &GetMosaicInfos) -> Result<Option<Vec<MosaicInfo>>> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_mosaic_infos(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}

pub fn get_mosaics_names(params: &GetMosaicsNames) -> Result<Option<Vec<MosaicName>>> {
	let fn_param = serde_json::to_vec(&params);
	if fn_param.is_err() {
		return Err(Error::SerializeJson);
	}

	let fn_params_body = fn_param.unwrap();
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_mosaics_names(fn_params_body.as_ptr(), fn_params_body.len(), fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};
	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}
