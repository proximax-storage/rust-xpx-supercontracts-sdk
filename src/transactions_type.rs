//! # Basic Transactions types for Blockchain functions and getters
//!
//! Most common representations for Blockchain transactions type,
//! traits, and informations getters.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const FUNCTION_RETURN_SUCCESS: i64 = 0;
pub const FUNCTION_CONSTRUCTOR_FAIL_TO_SAVE: i64 = -39;
pub const FUNCTION_ERROR: i64 = -1000;

pub type Cid = String;
pub type NetworkType = u8;
pub type EntityType = u16;
pub type EntityVersion = u32;
pub type Amount = i64;
pub type Deadline = i64;
pub type Height = i64;
pub type Hash = String;
pub type Address = String;
pub type NamespaceId = u64;
pub type AliasActionType = u8;
pub type MosaicId = u64;
pub type PubKey = String;
pub type AssetId = u64;
pub type Duration = i64;
pub type MosaicSupplyType = u8;
pub type MetadataModificationType = u8;
pub type HashType = u8;
pub type Message = String;
pub type OfferType = u8;
pub type TransactionID = String;
pub type TransactionType = u8;
pub type Signature = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Drive {
	pub drive: Cid,
	pub owner: PubKey,
	pub root: Cid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuperContract {
	pub id: Cid,
	pub drive: Drive,
	pub file: Cid,
	pub vmversion: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AbstractTransaction {
	pub height: Height,
	pub index: u32,
	pub transaction_hash: Option<Hash>,
	pub merkle_component_hash: Option<Hash>,
	pub aggregate_hash: Option<Hash>,
	pub unique_aggregate_hash: Option<Hash>,
	pub aggregate_id: Vec<u8>,

	pub network_type: NetworkType,
	pub deadline: Option<Deadline>,
	pub entity_type: EntityType,
	pub version: EntityVersion,
	pub max_fee: Amount,
	pub signature: Vec<u8>,
	pub signer: Option<PublicAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicDefinition {
	pub nonce: u32,
	pub owner_public_key: String,
	pub mosaic_props: Option<MosaicProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicProperties {
	pub supply_mutable: bool,
	pub transferable: bool,
	pub divisibility: u8,
	pub optional_properties: Vec<MosaicProperty>,
}

pub type MosaicPropertyId = u8;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicProperty {
	pub id: MosaicPropertyId,
	pub value: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddressAlias {
	pub address: Option<Address>,
	pub namespace_id: Option<NamespaceId>,
	pub action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicAlias {
	pub mosaic_id: Option<MosaicId>,
	pub namespace_id: Option<NamespaceId>,
	pub action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddExchangeOffer {
	pub add_offers: Option<Vec<AddOffer>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeConfirmation {
	#[serde(rename = "Type")]
	pub offer_type: OfferType,
	pub mosaic: Option<Mosaic>,
	pub cost: Amount,
	pub owner: Option<PublicAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddOffer {
	#[serde(rename = "Type")]
	pub offer_type: OfferType,
	pub mosaic: Option<Mosaic>,
	pub cost: Amount,
	pub duration: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeOffer {
	pub offer: Option<Vec<ExchangeConfirmation>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transfer {
	pub pub_key: PubKey,
	pub asset_id: AssetId,
	pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveOffer {
	#[serde(rename = "Type")]
	pub offer_type: OfferType,
	pub asset_id: AssetId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveExchangeOffer {
	pub remove_offers: Option<Vec<RemoveOffer>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicSupplyChange {
	pub asset_id: AssetId,
	pub supply_type: MosaicSupplyType,
	pub delta: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterRootNamespace {
	pub namespace_name: String,
	pub duration: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterSubNamespace {
	pub namespace_name: String,
	pub parent_id: Option<NamespaceId>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mosaic {
	pub asset_id: AssetId,
	pub amount: Amount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
	pub hash: Hash,
	pub hash_type: HashType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretLock {
	pub mosaic: Option<Mosaic>,
	pub duration: Duration,
	pub secret: Option<Secret>,
	pub recipient: Option<Address>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proof {
	pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretProof {
	pub hash_type: HashType,
	pub proof: Option<Proof>,
	pub recipient: Option<Address>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransferWithNamespace {
	pub recipient: Option<NamespaceId>,
	pub mosaics: Option<Vec<Mosaic>>,
	pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyMetadataAddress {
	pub address: Option<Address>,
	pub modifications: Option<Vec<MetadataModification>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModifyMetadataMosaic {
	pub mosaic_id: Option<MosaicId>,
	pub modifications: Option<Vec<MetadataModification>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetadataModification {
	#[serde(rename = "Type")]
	pub modification_type: MetadataModificationType,
	pub key: String,
	pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModifyMetadataNamespace {
	pub namespace_id: Option<NamespaceId>,
	pub modifications: Option<Vec<MetadataModification>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OfferInfo {
	#[serde(rename = "Type")]
	pub offer_type: OfferType,
	pub owner: Option<PublicAccount>,
	pub mosaic: Option<Mosaic>,
	pub price_numerator: Amount,
	pub price_denominator: Amount,
	pub deadline: Height,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserExchangeInfo {
	pub owner: Option<PublicAccount>,
	pub offers: Option<HashMap<OfferType, HashMap<MosaicId, Option<OfferInfo>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAccountExchangeInfo {
	pub pub_key: Option<PubKey>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetExchangeOfferByAssetId {
	pub asset_id: AssetId,
	pub offer_type: OfferType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicAccount {
	pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicInfo {
	pub mosaic_id: Option<MosaicId>,
	pub supply: Amount,
	pub height: Height,
	pub owner: Option<PublicAccount>,
	pub revision: u32,
	pub properties: Option<MosaicProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMosaicInfo {
	pub mosaic_id: Option<MosaicId>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMosaicInfos {
	pub msc_ids: Option<Vec<MosaicId>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MosaicName {
	pub mosaic_id: Option<MosaicId>,
	pub names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMosaicsNames {
	pub msc_ids: Option<Vec<MosaicId>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTransaction {
	pub id: TransactionID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactions {
	pub ids: Vec<TransactionID>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionStatus {
	pub deadline: Option<Deadline>,
	pub group: String,
	pub status: String,
	pub hash: Option<Hash>,
	pub height: Height,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTransactionStatus {
	pub id: TransactionID,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTransactionStatuses {
	pub ids: Vec<TransactionID>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTransactionEffectiveFee {
	pub id: TransactionID,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetMosaicID {
	pub nonce: u32,
	pub owner_public_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetInitiatorPubKey {
	pub public_key: String,
}

/// # Transactions
/// **SignedTransaction** is common interface for transactions data
/// from Blockchain.
pub trait SignedTransaction {
	fn id(&self) -> TransactionID;
	fn transaction_type(&self) -> TransactionType;
	fn signer(&self) -> PubKey;
	fn signature(&self) -> Signature;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
	pub hash: Hash,
	pub action_type: i64,
	pub size: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriveFsTransaction {
	pub id: TransactionID,
	pub transaction_type: TransactionType,
	pub signer: PubKey,
	pub signature: Signature,
	pub drive_id: Hash,
	pub add_actions: Option<Vec<Action>>,
	pub remove_actions: Option<Vec<Action>>,
}

impl DriveFsTransaction {
	pub fn drive_id(&self) -> Hash {
		self.drive_id.clone()
	}

	pub fn add_actions(&self) -> Option<Vec<Action>> {
		self.add_actions.clone()
	}

	pub fn remove_actions(&self) -> Option<Vec<Action>> {
		self.remove_actions.clone()
	}
}

impl SignedTransaction for DriveFsTransaction {
	fn id(&self) -> TransactionID {
		self.id.clone()
	}

	fn transaction_type(&self) -> TransactionType {
		self.transaction_type
	}

	fn signer(&self) -> PubKey {
		self.signer.clone()
	}

	fn signature(&self) -> Signature {
		self.signature.clone()
	}
}
