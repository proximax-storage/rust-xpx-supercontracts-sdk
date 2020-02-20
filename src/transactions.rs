//! Basic Blockchain functions and getters

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

#[derive(Debug, Deserialize, Serialize)]
pub struct AbstractTransaction {
	pub height: Height,
	pub index: uint32,
	pub transaction_hash: Option<Hash>,
	pub merkle_component_hash: Option<Hash>,
	pub aggregate_hash: Option<Hash>,
	pub unique_aggregate_hash: Option<Hash>,
	pub aggregate_id: [u8; 32],

	pub network_type: NetworkType,
	pub deadline: Option<Deadline>,
	pub entity_type: EntityType,
	pub version: EntityVersion,
	pub max_fee: Amount,
	pub signature: [u8; 64],
	pub signer: Option<PublicAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicDefinition {
	pub nonce: u32,
	pub owner_public_key: [u32],
	pub mosaic_props: Option<MosaicProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicProperties {
	pub supply_mutable: bool,
	pub transferable: bool,
	pbb divisibility: u8,
	pub optional_properties: []MosaicProperty,
}

pub type MosaicPropertyId = u8;

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicProperty {
	pub id: MosaicPropertyId,
	pub value: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddressAlias {
	address: Option<Address>,
	namespace_id: Option<NamespaceId>,
	action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicAlias {
	mosaic_id: Option<MosaicId>,
	namespace_id: Option<NamespaceId>,
	action_type: AliasActionType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddExchangeOffer {
	add_offers: Option< []AddOffer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeOffer {
	offer: Option<[]ExchangeConfirmation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {
	pub_key: PubKey,
	asset_id: AssetId,
	amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveExchangeOffer {
	remove_offers: Option<[]RemoveOffer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MosaicSupplyChange{
	asset_id: AssetId,
	supply_type: MosaicSupplyType,
	delta: Duration,
}
