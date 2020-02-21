//! Basic Blockchain functions and getters

use serde::de::DeserializeOwned;

use crate::external;
use crate::statuses::{FunctionResult, Result};
use crate::tools::call::call_external_func;
use crate::transactions_type::*;

pub fn flush() -> FunctionResult {
	return unsafe { Ok(external::flush()) };
}

pub fn mosaic_definition(params: &MosaicDefinition) -> Result<MosaicDefinitionTransaction> {
	call_external_func(params, external::mosaic_definition)
}

pub fn get_account_exchange_info(params: &GetAccountExchangeInfo) -> Result<UserExchangeInfo> {
	call_external_func(params, external::get_account_exchange_info)
}

pub fn get_exchange_offer_by_asset_id(params: &GetExchangeOfferByAssetId) -> Result<OfferInfo> {
	call_external_func(params, external::get_exchange_offer_by_asset_id)
}

pub fn get_mosaic_info(params: &GetMosaicInfo) -> Result<Option<MosaicInfo>> {
	call_external_func(params, external::get_mosaic_info)
}

pub fn get_mosaic_infos(params: &GetMosaicInfos) -> Result<Option<Vec<MosaicInfo>>> {
	call_external_func(params, external::get_mosaic_infos)
}

pub fn get_mosaics_names(params: &GetMosaicsNames) -> Result<Option<Vec<MosaicName>>> {
	call_external_func(params, external::get_mosaics_names)
}

pub fn get_transaction<T: SignedTransaction + DeserializeOwned>(params: &GetTransaction) -> Result<T> {
	call_external_func(params, external::get_transaction)
}

pub fn get_transactions<T: SignedTransaction + DeserializeOwned>(params: &GetTransactions) -> Result<Vec<T>> {
	call_external_func(params, external::get_transactions)
}

pub fn get_transaction_status(params: &GetTransactionStatus) -> Result<Option<TransactionStatus>> {
	call_external_func(params, external::get_transaction_status)
}

pub fn get_transaction_statuses(params: &GetTransactionStatuses) -> Result<Option<Vec<TransactionStatus>>> {
	call_external_func(params, external::get_transaction_statuses)
}

pub fn get_transaction_effective_fee(params: &GetTransactionEffectiveFee) -> Result<i64> {
	call_external_func(params, external::get_transaction_effective_fee)
}
