use serde::Serialize;

use xpx_supercontracts_sdk::statuses::Result;
use xpx_supercontracts_sdk::storage::save_result;
use xpx_supercontracts_sdk::transactions as tx;
use xpx_supercontracts_sdk::transactions::{get_supercontract, mosaic_definition};
use xpx_supercontracts_sdk::transactions_type::{AddExchangeOffer, AddOffer, AddressAlias, DriveFsTransaction, ExchangeConfirmation, ExchangeOffer, GetAccountExchangeInfo, GetExchangeOfferByAssetId, GetMosaicInfo, GetMosaicInfos, GetMosaicsNames, GetTransaction, GetTransactionEffectiveFee, GetTransactionStatus, GetTransactionStatuses, MetadataModification, ModifyMetadataAddress, ModifyMetadataMosaic, ModifyMetadataNamespace, Mosaic, MosaicAlias, MosaicDefinition, MosaicProperties, MosaicProperty, MosaicSupplyChange, Proof, PublicAccount, RegisterRootNamespace, RegisterSubNamespace, RemoveExchangeOffer, RemoveOffer, Secret, SecretLock, SecretProof, SuperContract, Transfer, TransferWithNamespace};
use xpx_supercontracts_sdk::utils::{constructor, debug_message, init, ping};

#[no_mangle]
pub extern "C" fn ping100() -> i64 {
	ping(100).unwrap()
}

#[no_mangle]
pub extern "C" fn ping100init() -> i64 {
	let _ = init(|| {
		ping(100).unwrap();
	});
	let _ = init(|| {
		ping(100).unwrap();
	});
	ping(100).unwrap()
}

#[no_mangle]
pub extern "C" fn ping_with_params(x: i64) -> i64 {
	ping(x as usize).unwrap()
}

#[no_mangle]
pub extern "C" fn ping100constructor() -> i64 {
	let _ = constructor(|| ping(100).unwrap());
	let _ = constructor(|| ping(100).unwrap());
	ping(100).unwrap()
}

#[no_mangle]
pub extern "C" fn save_sample_result() -> i64 {
	#[derive(Serialize)]
	struct SampleData<'a> {
		pub id: i64,
		pub message: &'a str,
	}
	let data = serde_json::to_vec(&SampleData {
		id: 10,
		message: "awesome message",
	})
		.unwrap();
	let res = save_result(&"data.txt".to_string(), &data[..]);
	if let Err(err) = res {
		debug_message(&format!("save_result error {:?}", err));
		return -1;
	}
	res.unwrap()
}

#[no_mangle]
pub extern "C" fn exmpl_get_supercontract() -> i64 {
	let res = tx::get_supercontract();
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("SC.ID: {:?}", res.unwrap().id));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_transaction_effective_fee() -> i64 {
	let res = tx::get_transaction_effective_fee(&GetTransactionEffectiveFee {
		id: String::from("baegqajaiaqjcb6qufter6rgkiyndqd273kp4n43q2q34uhbt24qb3ehexqcu74u3"),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_transaction() -> i64 {
	let tx_result: Result<DriveFsTransaction> = tx::get_transaction(&GetTransaction {
		id: String::from("some_id"),
	});
	if let Err(err) = tx_result {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", tx_result.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_transaction_status() -> i64 {
	let res = tx::get_transaction_status(&GetTransactionStatus {
		id: String::from("some_id"),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_transaction_statuses() -> i64 {
	let res = tx::get_transaction_statuses(&GetTransactionStatuses {
		ids: vec![String::from("some_id")],
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_account_exchange_info() -> i64 {
	let res = tx::get_account_exchange_info(&GetAccountExchangeInfo {
		pub_key: Some(String::from("some_pub_key")),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_exchange_offer_by_asset_id() -> i64 {
	let res = tx::get_exchange_offer_by_asset_id(&GetExchangeOfferByAssetId {
		asset_id: 1,
		offer_type: 2,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_mosaic_info() -> i64 {
	let res = tx::get_mosaic_info(&GetMosaicInfo { mosaic_id: Some(1) });
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_mosaic_infos() -> i64 {
	let res = tx::get_mosaic_infos(&GetMosaicInfos {
		msc_ids: Some(vec![2]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_get_mosaics_names() -> i64 {
	let res = tx::get_mosaics_names(&GetMosaicsNames {
		msc_ids: Some(vec![1]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_modify_metadata_namespace() -> i64 {
	let res = tx::modify_metadata_namespace(&ModifyMetadataNamespace {
		namespace_id: Some(1),
		modifications: Some(vec![MetadataModification {
			modification_type: 1,
			key: String::from("some_key"),
			value: String::from("some_key"),
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_modify_metadata_mosaic() -> i64 {
	let res = tx::modify_metadata_mosaic(&ModifyMetadataMosaic {
		mosaic_id: Some(1),
		modifications: Some(vec![MetadataModification {
			modification_type: 3,
			key:  String::from("10"),
			value: String::from("20"),
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_modify_metadata_address() -> i64 {
	let res = tx::modify_metadata_address(&ModifyMetadataAddress {
		address: Some(String::from("some_address")),
		modifications: Some(vec![MetadataModification {
			modification_type: 3,
			key:  String::from("10"),
			value: String::from("20"),
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_secret_proof() -> i64 {
	let res = tx::secret_proof(&SecretProof {
		hash_type: 1,
		proof: Some(Proof {
			data: vec![1],
		}),
		recipient: Some(String::from("some recipient")),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_transfer_with_namespace() -> i64 {
	let res = tx::transfer_with_namespace(&TransferWithNamespace {
		recipient: Some(1),
		mosaics: Some(vec![Mosaic {
			asset_id: 4,
			amount: 10000,
		}]),
		message: String::from("some message"),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_secret_lock() -> i64 {
	let res = tx::secret_lock(&SecretLock {
		mosaic: Some(Mosaic {
			asset_id: 1,
			amount: 1000,
		}),
		duration: 10000,
		secret: Some(Secret {
			hash: String::from("some hash"),
			hash_type: 1,
		}),
		recipient: Some(String::from("some addr")),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_register_sub_namespace() -> i64 {
	let res = tx::register_sub_namespace(&RegisterSubNamespace {
		namespace_name: String::from("name"),
		parent_id: Some(3),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_register_root_namespace() -> i64 {
	let res = tx::register_root_namespace(&RegisterRootNamespace {
		namespace_name: String::from("name"),
		duration: 1000,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_mosaic_supply_change() -> i64 {
	let res = tx::mosaic_supply_change(&MosaicSupplyChange {
		asset_id: 10,
		supply_type: 100,
		delta: 300,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_remove_exchange_offer() -> i64 {
	let res = tx::remove_exchange_offer(&RemoveExchangeOffer {
		remove_offers: Some(vec![RemoveOffer {
			asset_id: 10,
			offer_type: 1,
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_transfer() -> i64 {
	let res = tx::transfer(&Transfer {
		pub_key: String::from("some_pub_key"),
		asset_id: 10,
		amount: 1000,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_exchange_offer() -> i64 {
	let res = tx::exchange_offer(&ExchangeOffer {
		offer: Some(vec![ExchangeConfirmation {
			offer_type: 1,
			mosaic: Some(Mosaic {
				asset_id: 3,
				amount: 10000,
			}),
			cost: 1000,
			owner: Some(PublicAccount {
				public_key: String::from(
					"baegqajaiaqjcakxppt6hi3pcghh3en7fj47jkf6phllbkz2hx2bdvdeyelrrcmx5",
				),
			}),
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_add_exchange_offer() -> i64 {
	let res = tx::add_exchange_offer(&AddExchangeOffer {
		add_offers: Some(vec![AddOffer {
			offer_type: 3,
			mosaic: Some(Mosaic {
				amount: 1000,
				asset_id: 3,
			}),
			cost: 100,
			duration: 1000,
		}]),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_mosaic_alias() -> i64 {
	let res = tx::mosaic_alias(&MosaicAlias {
		mosaic_id: Some(10),
		namespace_id: Some(3),
		action_type: 2,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_address_alias() -> i64 {
	let res = tx::address_alias(&AddressAlias {
		address: Some(String::from("SAONSOGFZZHNEIBRYXHDTDTBR2YSAXKTITRFHG2Y")),
		namespace_id: Some(0xFF000000_FF000000),
		action_type: 2,
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}

#[no_mangle]
pub extern "C" fn exmpl_create_mosaic() -> i64 {
	let res = get_supercontract();
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	let sc: SuperContract = res.unwrap();

	debug_message(&format!("SC.ID: {:?}", sc.id));
	let res = mosaic_definition(&MosaicDefinition {
		nonce: 0,
		owner_public_key: sc.id,
		mosaic_props: Some(MosaicProperties {
			supply_mutable: true,
			transferable: true,
			divisibility: 0,
			optional_properties: vec![MosaicProperty { id: 0, value: 0 }],
		}),
	});
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}

	debug_message(&format!("Result: {:?}", res.unwrap()));
	0
}
