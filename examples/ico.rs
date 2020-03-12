use xpx_supercontracts_sdk::utils::constructor;

mod ico {
	use serde::Deserialize;

	use xpx_supercontracts_sdk::storage::storage_get;
	use xpx_supercontracts_sdk::transactions::{flush, get_supercontract, mosaic_definition};
	use xpx_supercontracts_sdk::transactions_type::{FUNCTION_ERROR, FUNCTION_RETURN_SUCCESS, MosaicDefinition, MosaicProperties, MosaicProperty, SuperContract};
	use xpx_supercontracts_sdk::utils::debug_message;

	const ICO_CSV_FILE: &str = "ico_init.csv";

	#[derive(Debug, Deserialize)]
	struct CsvIcoData {
		name: Option<String>,
		shareholder_address: String,
		amount: u64,
	}

	pub fn create_ico() -> i64 {
		let mosaic_result = create_mosaic();
		if mosaic_result < FUNCTION_RETURN_SUCCESS {
			return mosaic_result;
		}

		let file_result = storage_get(&ICO_CSV_FILE.to_string());
		if let Err(err) = file_result {
			debug_message(&format!("failed load CSV file: {}", err));
			return FUNCTION_ERROR;
		}
		0
	}

	fn parse_csv(data: &Vec<u8>) -> Vec<CsvIcoData> {
		let mut csv_reader = csv::ReaderBuilder::new()
			.delimiter(b';')
			.from_reader(&data[..]);
		vec![]
	}

	fn create_mosaic() -> i64 {
		let res = get_supercontract();
		if res.is_err() {
			return FUNCTION_ERROR;
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
				optional_properties: vec![MosaicProperty {
					id: 0,
					value: 0,
				}],
			}),
		});
		if let Err(err) = res {
			debug_message(&format!("failed create mosaic: {}", err));
			return FUNCTION_ERROR;
		}
		let result = res.unwrap();
		if result < FUNCTION_RETURN_SUCCESS {
			debug_message(&"failed create mosaic".to_string());
			return result;
		}

		// Flush Tx
		let res = flush();
		if let Err(err) = res {
			debug_message(&format!("failed flush transaction: {}", err));
			return FUNCTION_ERROR;
		}

		FUNCTION_RETURN_SUCCESS
	}
}

#[no_mangle]
pub extern "C" fn ico_init() -> i64 {
	constructor(ico::create_ico)
}
