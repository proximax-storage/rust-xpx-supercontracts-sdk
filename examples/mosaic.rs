use xpx_supercontracts_sdk::{
	transactions::mosaic_definition,
	transactions_type::{MosaicDefinition, MosaicProperties, MosaicProperty},
	utils::{constructor, debug_message},
};
use xpx_supercontracts_sdk::transactions::get_supercontract;
use xpx_supercontracts_sdk::transactions_type::{FUNCTION_CONSTRUCTOR_FAIL_TO_SAVE, FUNCTION_ERROR, FUNCTION_RETURN_SUCCESS, SuperContract};

pub fn create_mosaic() -> i64 {
	let res = get_supercontract();
	if res.is_err() {
		return -1;
	}
	let sc: SuperContract = res.unwrap();

	debug_message(&format!("SC.ID: {:?}", sc.drive.owner));
	let res = mosaic_definition(&MosaicDefinition {
		nonce: 0,
		owner_public_key: sc.drive.owner,
		mosaic_props: Some(MosaicProperties {
			supply_mutable: true,
			transferable: true,
			divisibility: 0,
			optional_properties: vec![MosaicProperty {
				id: 0,
				value: 0,
			}],
		}),
	});
	if res.is_err() {
		return FUNCTION_ERROR;
	}
	if res.unwrap() < FUNCTION_RETURN_SUCCESS {
		debug_message(&"failed create mosaic".to_string());
	}
	FUNCTION_RETURN_SUCCESS
}

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
	let res = constructor(create_mosaic);
	if res != FUNCTION_RETURN_SUCCESS && res == FUNCTION_CONSTRUCTOR_FAIL_TO_SAVE {
		return res;
	}
	FUNCTION_RETURN_SUCCESS
}
