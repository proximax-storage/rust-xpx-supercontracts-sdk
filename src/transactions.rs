//! # Basic Blockchain functions
//! Basic functionality to interact with Catapulta Blockchain. 
//! It represent via twa main part: transactions 
//! and getters (**get functions** of some information from Blockchain).
//! 
//! ## Transaction functions
//! Main principle of transaction functions is 
//! that aggregation to pool of transactions. So it will send not immediately. 
//! The reason is minimize `Gas spent` and reducing Blockchain resources utilization.
//! 
//! When SuperContract will End transactions Flush autromaticaly (it mean it will send 
//! to Blocchain).
//! 
//! But it possible send transaction pull immediately.
//! For that case exist function `flush`.
//! When function `flush` invoking then Transactions Pool 
//! will be send to Blockchain, and then clear.
//! 
//! And again - only **end execution of SuperContract**
//!  or Flush** can send transactions to Blockchaing. 
//! Otherwise it will gather to Transactions Pool.

use serde::de::DeserializeOwned;

use crate::external;
use crate::statuses::{FunctionResult, Result};
use crate::statuses::Error;
use crate::tools::call::{call_external_func, simple_call_external_func};
use crate::transactions_type::*;

/// Flush send immediately Transactions from current 
/// transactions pool.
/// 
/// Flush should use carefully. If Leader of SuperContract 
/// execution invoke Flush then SuperContract will be
/// paused ann waite 
/// Signing from other Executors.
/// When Flush will send from non-leader it mean fire 
/// process of signing transactions 
/// from Leader of SuperContract execution.
/// In both case SuperContract will be paused waiting
/// event from other Executors.
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		mosaic_definition,
/// };
/// use xpx_supercontracts_sdk::transactions_type::MosaicDefinition;
/// 
/// let pub_key = "2C8178EF9ED7A6D30ABDC1E4D30D68B05861112A98B1629FBE2C8D16FDE97A1C".as_bytes().to_vec();
/// let params = MosaicDefinition{
///		nonce: 100,
///		owner_public_key: pub_key,
///		mosaic_props: None, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = mosaic_definition(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// // Flush current Transactions Pool
/// let flush_result = flush();
/// if flush_result.is_err() {
///     // Some error handling
/// }
/// ```
/// 
pub fn flush() -> FunctionResult {
	return unsafe { Ok(external::flush()) };
}

/// Send to transaction pool **MosaicDefinition transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		mosaic_definition,
/// };
/// use xpx_supercontracts_sdk::transactions_type::MosaicDefinition;
/// 
/// let pub_key = "2C8178EF9ED7A6D30ABDC1E4D30D68B05861112A98B1629FBE2C8D16FDE97A1C".as_bytes().to_vec();
/// let params = MosaicDefinition{
///		nonce: 100,
///		owner_public_key: pub_key,
///		mosaic_props: None, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = mosaic_definition(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn mosaic_definition(params: &MosaicDefinition) -> FunctionResult {
	simple_call_external_func(params, external::mosaic_definition)
}

/// Send to transaction pool **AddressAlias transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		address_alias,
/// };
/// use xpx_supercontracts_sdk::transactions_type::AddressAlias;
/// 
/// let params = AddressAlias{
///		address: None,
///		namespace_id: None,
///		action_type: 10, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = address_alias(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn address_alias(params: &AddressAlias) -> FunctionResult {
	simple_call_external_func(params, external::address_alias)
}

/// Send to transaction pool **MosaicAlias transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		mosaic_alias,
/// };
/// use xpx_supercontracts_sdk::transactions_type::MosaicAlias;
/// 
/// let params = MosaicAlias{
///		mosaic_id: None,
///		namespace_id: None,
///		action_type: 10, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = mosaic_alias(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn mosaic_alias(params: &MosaicAlias) -> FunctionResult {
	simple_call_external_func(params, external::mosaic_alias)
}

/// Send to transaction pool **AddExchangeOffer transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		add_exchange_offer,
/// };
/// use xpx_supercontracts_sdk::transactions_type::AddExchangeOffer;
/// 
/// let params = AddExchangeOffer{
///		add_offers: None, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = add_exchange_offer(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn add_exchange_offer(params: &AddExchangeOffer) -> FunctionResult {
	simple_call_external_func(params, external::add_exchange_offer)
}

/// Send to transaction pool **AddExchangeOffer transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		add_exchange_offer,
/// };
/// use xpx_supercontracts_sdk::transactions_type::AddExchangeOffer;
/// 
/// let params = AddExchangeOffer{
///		add_offers: None, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = add_exchange_offer(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn exchange_offer(params: &ExchangeOffer) -> FunctionResult {
	simple_call_external_func(params, external::exchange_offer)
}

/// Send to transaction pool **Transfer transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		transfer,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{Transfer, PubKey};
/// 
/// let pub_key = "2C8178EF9ED7A6D30ABDC1E4D30D68B05861112A98B1629FBE2C8D16FDE97A1C".as_bytes();
/// let mut pk: PubKey = [0; 32];
/// pk.copy_from_slice(&pub_key[..32]);
/// let params = Transfer{
/// 	pub_key: pk, 
///		asset_id: 10,
///		amount: 1000, 	
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = transfer(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn transfer(params: &Transfer) -> FunctionResult {
	simple_call_external_func(params, external::transfer)
}

/// Send to transaction pool **RemoveExchangeOffer transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		remove_exchange_offer,
/// };
/// use xpx_supercontracts_sdk::transactions_type::RemoveExchangeOffer;
///
/// let params = RemoveExchangeOffer{
/// 	remove_offers: None, 
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = remove_exchange_offer(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn remove_exchange_offer(params: &RemoveExchangeOffer) -> FunctionResult {
	simple_call_external_func(params, external::remove_exchange_offer)
}

/// Send to transaction pool **MosaicSupplyChange transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		mosaic_supply_change,
/// };
/// use xpx_supercontracts_sdk::transactions_type::MosaicSupplyChange;
///
/// let params = MosaicSupplyChange{
/// 	asset_id: 10,
///		supply_type: 20,
///		delta: 300, 
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = mosaic_supply_change(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn mosaic_supply_change(params: &MosaicSupplyChange) -> FunctionResult {
	simple_call_external_func(params, external::mosaic_supply_change)
}

/// Send to transaction pool **RegisterRootNamespace transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		register_root_namespace,
/// };
/// use xpx_supercontracts_sdk::transactions_type::RegisterRootNamespace;
///
/// let params = RegisterRootNamespace{
/// 	namespace_name: String::from("my_name_space"),
///		duration: 3000, 
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = register_root_namespace(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn register_root_namespace(params: &RegisterRootNamespace) -> FunctionResult {
	simple_call_external_func(params, external::register_root_namespace)
}

/// Send to transaction pool **RegisterRootNamespace transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		register_root_namespace,
/// };
/// use xpx_supercontracts_sdk::transactions_type::RegisterRootNamespace;
///
/// let params = RegisterRootNamespace{
/// 	namespace_name: String::from("my_name_space"),
///		duration: 3000, 
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = register_root_namespace(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn register_sub_namespace(params: &RegisterSubNamespace) -> FunctionResult {
	simple_call_external_func(params, external::register_sub_namespace)
}

/// Send to transaction pool **SecretLock transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		secret_lock,
/// };
/// use xpx_supercontracts_sdk::transactions_type::SecretLock;
///
/// let params = SecretLock{
/// 	mosaic: None,
///		duration: 3000,
///		secret: None,
///		recipient: None, 
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = secret_lock(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn secret_lock(params: &SecretLock) -> FunctionResult {
	simple_call_external_func(params, external::secret_lock)
}

/// Send to transaction pool **SecretProof transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		secret_proof,
/// };
/// use xpx_supercontracts_sdk::transactions_type::SecretProof;
///
/// let params = SecretProof{
/// 	hash_type: 10,
///		proof: None,
///		recipient: None,
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = secret_proof(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn secret_proof(params: &SecretProof) -> FunctionResult {
	simple_call_external_func(params, external::secret_proof)
}

/// Send to transaction pool **TransferWithNamespace transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		transfer_with_namespace,
/// };
/// use xpx_supercontracts_sdk::transactions_type::TransferWithNamespace;
///
/// let params = TransferWithNamespace{
/// 	recipient: None,
///		mosaics: None,
///		message: "some message".as_bytes().to_vec(),
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = transfer_with_namespace(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn transfer_with_namespace(params: &TransferWithNamespace) -> FunctionResult {
	simple_call_external_func(params, external::transfer_with_namespace)
}

/// Send to transaction pool **ModifyMetadataAddress transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		modify_metadata_address,
/// };
/// use xpx_supercontracts_sdk::transactions_type::ModifyMetadataAddress;
///
/// let params = ModifyMetadataAddress{
/// 	address: None,
///		modifications: None,
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = modify_metadata_address(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn modify_metadata_address(params: &ModifyMetadataAddress) -> FunctionResult {
	simple_call_external_func(params, external::modify_metadata_address)
}

/// Send to transaction pool **ModifyMetadataMosaic transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		modify_metadata_mosaic,
/// };
/// use xpx_supercontracts_sdk::transactions_type::ModifyMetadataMosaic;
///
/// let params = ModifyMetadataMosaic{
/// 	mosaic_id: None,
///		modifications: None,
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = modify_metadata_mosaic(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn modify_metadata_mosaic(params: &ModifyMetadataMosaic) -> FunctionResult {
	simple_call_external_func(params, external::modify_metadata_mosaic)
}

/// Send to transaction pool **ModifyMetadataNamespace transaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		modify_metadata_namespace,
/// };
/// use xpx_supercontracts_sdk::transactions_type::ModifyMetadataNamespace;
///
/// let params = ModifyMetadataNamespace{
/// 	namespace_id: Some(10),
///		modifications: None,
/// };
/// // Add transaction to Transactions Pool
/// let tx_result = modify_metadata_namespace(&params);
/// if tx_result.is_err() {
///     // Some error handling
/// }
/// ```
///
pub fn modify_metadata_namespace(params: &ModifyMetadataNamespace) -> FunctionResult {
	simple_call_external_func(params, external::modify_metadata_namespace)
}

/// Get data via **GetAccountExchangeInfo**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		get_account_exchange_info,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetAccountExchangeInfo, UserExchangeInfo};
///
/// let params = GetAccountExchangeInfo{
/// 	pub_key: None,
/// };
/// // Get info data
/// let result = get_account_exchange_info(&params);
/// if result.is_err() {
///     // Some error handling
/// }
/// let info: UserExchangeInfo = result.unwrap(); 
/// ```
///
pub fn get_account_exchange_info(params: &GetAccountExchangeInfo) -> Result<UserExchangeInfo> {
	call_external_func(params, external::get_account_exchange_info)
}

/// Get data via **GetExchangeOfferByAssetId**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		get_exchange_offer_by_asset_id,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetExchangeOfferByAssetId, OfferInfo};
///
/// let params = GetExchangeOfferByAssetId{
/// 	asset_id: 10,
/// 	offer_type: 20,
/// };
/// // Get info data
/// let result = get_exchange_offer_by_asset_id(&params);
/// if result.is_err() {
///     // Some error handling
/// }
/// let info: OfferInfo = result.unwrap(); 
/// ```
///
pub fn get_exchange_offer_by_asset_id(params: &GetExchangeOfferByAssetId) -> Result<OfferInfo> {
	call_external_func(params, external::get_exchange_offer_by_asset_id)
}

/// Get data via **GetMosaicInfo**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		get_mosaic_info,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetMosaicInfo, MosaicInfo};
///
/// let params = GetMosaicInfo{
/// 	mosaic_id: None,
/// };
/// // Get info data
/// let result = get_mosaic_info(&params);
/// if result.is_err() {
///     // Some error handling
/// }
/// let info: Option<MosaicInfo> = result.unwrap(); 
/// ```
///
pub fn get_mosaic_info(params: &GetMosaicInfo) -> Result<Option<MosaicInfo>> {
	call_external_func(params, external::get_mosaic_info)
}

/// Get data via **GetMosaicInfos**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		get_mosaic_infos,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetMosaicInfos, MosaicInfo};
///
/// let params = GetMosaicInfos{
/// 	msc_ids: None,
/// };
/// // Get info data
/// let result = get_mosaic_infos(&params);
/// if result.is_err() {
///     // Some error handling
/// }
/// let info: Option<Vec<MosaicInfo>> = result.unwrap(); 
/// ```
///
pub fn get_mosaic_infos(params: &GetMosaicInfos) -> Result<Option<Vec<MosaicInfo>>> {
	call_external_func(params, external::get_mosaic_infos)
}

/// Get data via **GetMosaicsNames**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		get_mosaics_names,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetMosaicsNames, MosaicName};
///
/// let params = GetMosaicsNames{
/// 	msc_ids: None,
/// };
/// // Get info data
/// let result = get_mosaics_names(&params);
/// if result.is_err() {
///     // Some error handling
/// }
/// let info: Option<Vec<MosaicName>> = result.unwrap(); 
/// ```
///
pub fn get_mosaics_names(params: &GetMosaicsNames) -> Result<Option<Vec<MosaicName>>> {
	call_external_func(params, external::get_mosaics_names)
}

/// Get data via **GetTransaction**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		get_transaction,
/// };
/// use xpx_supercontracts_sdk::statuses::Result;
/// use xpx_supercontracts_sdk::transactions_type::{GetTransaction, DriveFsTransaction};
///
/// let params = GetTransaction{
/// 	id: [0; 32],
/// };
/// // Get info data
/// let tx_result: Result<DriveFsTransaction> = get_transaction(&params);
/// if tx_result.is_err() {
/// 	// Handle error
/// }
/// let tx = tx_result.unwrap();
/// ```
///
pub fn get_transaction<T: SignedTransaction + DeserializeOwned>(params: &GetTransaction) -> Result<T> {
	call_external_func(params, external::get_transaction)
}

/// Get data via **GetTransactionStatus**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		flush,
///		get_transaction_status,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetTransactionStatus, TransactionStatus};
///
/// let params = GetTransactionStatus{
/// 	id: [0; 32],
/// };
/// // Get info data
/// let result = get_transaction_status(&params);
/// let info = result.unwrap();
/// ```
///
pub fn get_transaction_status(params: &GetTransactionStatus) -> Result<Option<TransactionStatus>> {
	call_external_func(params, external::get_transaction_status)
}

/// Get data via **GetTransactionStatuses**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		get_transaction_statuses,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetTransactionStatuses, TransactionStatus};
///
/// let params = GetTransactionStatuses{
/// 	ids: vec!([0;32]),
/// };
/// // Get info data
/// let result = get_transaction_statuses(&params);
/// let info: Option<Vec<TransactionStatus>> = result.unwrap();
/// ```
///
pub fn get_transaction_statuses(params: &GetTransactionStatuses) -> Result<Option<Vec<TransactionStatus>>> {
	call_external_func(params, external::get_transaction_statuses)
}

/// Get data via **GetTransactionEffectiveFee**
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		get_transaction_effective_fee,
/// };
/// use xpx_supercontracts_sdk::transactions_type::{GetTransactionEffectiveFee};
///
/// let params = GetTransactionEffectiveFee{
/// 	id: [0;32],
/// };
/// // Get info data
/// let result = get_transaction_effective_fee(&params);
/// let info = result.unwrap();
/// ```
///
pub fn get_transaction_effective_fee(params: &GetTransactionEffectiveFee) -> Result<i64> {
	call_external_func(params, external::get_transaction_effective_fee)
}

/// Get data current SuperContract data
/// 
/// ## Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::transactions::{
///		get_supercontract,
/// };
///
/// // Get SC data
/// let result = get_supercontract();
/// let sc = result.unwrap();
/// ```
///
pub fn get_supercontract() -> Result<SuperContract> {
	let fn_result = unsafe {
		let fn_result: &mut Vec<u8> = &mut vec![];
		let fn_result_len = external::get_supercontract(fn_result.as_mut_ptr());
		let fn_data_bytes = fn_result.get_unchecked_mut(0..fn_result_len as usize);
		fn_data_bytes.to_vec()
	};

	let result = serde_json::from_slice(&fn_result[..]);
	if result.is_err() {
		return Err(Error::DeserializeJson);
	}
	Ok(result.unwrap())
}
