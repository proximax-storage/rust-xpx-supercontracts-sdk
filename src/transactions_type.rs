//! # Basic Transactions types for Blockchain functions and getters
//! 
//! Most common representations for Blockchain transactions type,
//! traits, and informations getters.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type NetworkType = u8;
pub type EntityType = u16;
pub type EntityVersion = u32;
pub type Amount = i64;
pub type Deadline = i64;
pub type PublicAccount = [u8; 32];
pub type Height = i64;
pub type Hash = [u8; 32];
pub type Address = [u8; 25];
pub type NamespaceId = i64;
pub type AliasActionType = u8;
pub type MosaicId = i64;
pub type PubKey = [u8; 32];
pub type AssetId = u64;
pub type Duration = i64;
pub type MosaicSupplyType = u8;
pub type MetadataModificationType = u8;
pub type HashType = u8;
pub type Message = Vec<u8>;
pub type OfferType = u8;
pub type TransactionID = [u8; 32];
pub type TransactionType = u8;
pub type Signature = [u8; 64];

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
pub struct MosaicDefinition {
	pub nonce: u32,
	pub owner_public_key: Vec<u8>,
	pub mosaic_props: Option<MosaicProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicProperties {
	pub supply_mutable: bool,
	pub transferable: bool,
	pub divisibility: u8,
	pub optional_properties: Vec<MosaicProperty>,
}

pub type MosaicPropertyId = u8;

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicProperty {
	pub id: MosaicPropertyId,
	pub value: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddressAlias {
	pub address: Option<Address>,
	pub namespace_id: Option<NamespaceId>,
	pub action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicAlias {
	pub mosaic_id: Option<MosaicId>,
	pub namespace_id: Option<NamespaceId>,
	pub action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddExchangeOffer {
	pub add_offers: Option<Vec<AddOffer>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeConfirmation {
	offer_type: OfferType,
	mosaic: Option<Mosaic>,
	cost: Amount,
	owner: Option<PublicAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOffer {
	pub offer_type: OfferType,
	pub mosaic: Option<Mosaic>,
	pub cost: Amount,
	pub duration: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeOffer {
	pub offer: Option<Vec<ExchangeConfirmation>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {
	pub pub_key: PubKey,
	pub asset_id: AssetId,
	pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveOffer {
	offer_type: OfferType,
	asset_id: AssetId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveExchangeOffer {
	pub remove_offers: Option<Vec<RemoveOffer>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicSupplyChange {
	pub asset_id: AssetId,
	pub supply_type: MosaicSupplyType,
	pub delta: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRootNamespace {
	pub namespace_name: String,
	pub duration: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterSubNamespace {
	pub namespace_name: String,
	pub parent_id: NamespaceId,
}

#[derive(Debug, Deserialize, Serialize)]
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
pub struct SecretProof {
	pub hash_type: HashType,
	pub proof: Option<Proof>,
	pub recipient: Option<Address>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferWithNamespace {
	pub recipient: Option<NamespaceId>,
	pub mosaics: Option<Vec<Mosaic>>,
	pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
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
pub struct MetadataModification {
	modification_type: MetadataModificationType,
	key: String,
	value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModifyMetadataNamespace {
	pub namespace_id: NamespaceId,
	pub modifications: Option<Vec<MetadataModification>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OfferInfo {
	pub offer_type: OfferType,
	pub owner: Option<PublicAccount>,
	pub mosaic: Option<Mosaic>,
	pub price_numerator: Amount,
	pub price_denominator: Amount,
	pub deadline: Height,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserExchangeInfo {
	pub owner: Option<PublicAccount>,
	pub offers: HashMap<OfferType, HashMap<MosaicId, Option<OfferInfo>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccountExchangeInfo {
	pub account: Option<PublicAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetExchangeOfferByAssetId {
	pub asset_id: AssetId,
	pub offer_type: OfferType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicInfo {
	mosaic_id: Option<MosaicId>,
	supply: Amount,
	height: Height,
	owner: Option<PublicAccount>,
	revision: u32,
	properties: Option<MosaicProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMosaicInfo {
	pub mosaic_id: Option<MosaicId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMosaicInfos {
	pub msc_ids: Option<Vec<MosaicId>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicName {
	pub mosaic_id: Option<MosaicId>,
	pub names: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMosaicsNames {
	pub msc_ids: Option<Vec<MosaicId>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransaction {
	pub id: TransactionID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactions {
	pub ids: Vec<TransactionID>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionStatus {
	deadline: Option<Deadline>,
	group: String,
	status: String,
	hash: Option<Hash>,
	height: Height,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactionStatus {
	pub id: TransactionID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactionStatuses {
	pub ids: Vec<TransactionID>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactionEffectiveFee {
	pub id: TransactionID,
}

/// # Transactions
/// **SignedTransaction** is common interface for transactions data
/// from Blockchain. 
pub trait SignedTransaction {
	fn id(&self) -> TransactionID;
	fn transaction_type() -> TransactionType;
	fn signer() -> PubKey;
	fn signature() -> Vec<u8>;
}
