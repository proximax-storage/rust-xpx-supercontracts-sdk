//! Basic Blockchain functions and getters

use serde::de::DeserializeOwned;

use crate::external;
use crate::statuses::{FunctionResult, Result};
use crate::tools::call::call_external_func;
use crate::transactions_type::*;

pub fn flush() -> FunctionResult {
	return unsafe { Ok(external::flush()) };
}

pub fn mosaic_definition(params: &MosaicDefinition) -> FunctionResult {
	call_external_func(params, external::mosaic_definition)
}

pub fn address_alias(params: &AddressAlias) -> FunctionResult {
	call_external_func(params, external::address_alias)
}

pub fn mosaic_alias(params: &MosaicAlias) -> FunctionResult {
	call_external_func(params, external::mosaic_alias)
}

pub fn add_exchange_offer(params: &AddExchangeOffer) -> FunctionResult {
	call_external_func(params, external::add_exchange_offer)
}

pub fn exchange_offer(params: &ExchangeOffer) -> FunctionResult {
	call_external_func(params, external::exchange_offer)
}

pub fn transfer(params: &Transfer) -> FunctionResult {
	call_external_func(params, external::transfer)
}

pub fn remove_exchange_offer(params: &RemoveExchangeOffer) -> FunctionResult {
	call_external_func(params, external::remove_exchange_offer)
}

pub fn mosaic_supply_change(params: &MosaicSupplyChange) -> FunctionResult {
	call_external_func(params, external::mosaic_supply_change)
}

pub fn register_root_namespace(params: &RegisterRootNamespace) -> FunctionResult {
	call_external_func(params, external::register_root_namespace)
}

pub fn register_sub_namespace(params: &RegisterSubNamespace) -> FunctionResult {
	call_external_func(params, external::register_sub_namespace)
}

pub fn secret_lock(params: &SecretLock) -> FunctionResult {
	call_external_func(params, external::secret_lock)
}

pub fn secret_proof(params: &SecretProof) -> FunctionResult {
	call_external_func(params, external::secret_proof)
}

pub fn transfer_with_namespace(params: &TransferWithNamespace) -> FunctionResult {
	call_external_func(params, external::transfer_with_namespace)
}

pub fn modify_metadata_address(params: &ModifyMetadataAddress) -> FunctionResult {
	call_external_func(params, external::modify_metadata_address)
}

pub fn modify_metadata_mosaic(params: &ModifyMetadataMosaic) -> FunctionResult {
	call_external_func(params, external::modify_metadata_mosaic)
}

pub fn modify_metadata_namespace(params: &ModifyMetadataNamespace) -> FunctionResult {
	call_external_func(params, external::modify_metadata_namespace)
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
