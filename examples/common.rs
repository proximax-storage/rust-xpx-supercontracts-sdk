use serde::Serialize;

use xpx_supercontracts_sdk::utils::{constructor, init, ping, debug_message};
use xpx_supercontracts_sdk::transactions as tx;
use xpx_supercontracts_sdk::storage::save_result;

#[no_mangle]
pub extern "C" fn ping100() -> i64 {
	ping(100).unwrap()
}

#[no_mangle]
pub extern "C" fn ping100init() -> i64 {
	let _ = init(|| { ping(100).unwrap(); });
	let _ = init(|| { ping(100).unwrap(); });
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
	let data = serde_json::to_vec(&SampleData{
		id: 10,
		message: "awesome message",
	}).unwrap();
	let res = save_result(&"data.txt".to_string(), &data[..]);
	if let Err(err) = res {
		debug_message(&format!("save_result error {:?}", err));
		return -1;
	}
	res.unwrap()
}

#[no_mangle]
pub extern "C" fn get_sc() -> i64 {
	let res = tx::get_supercontract();
	if let Err(err) = res {
		debug_message(&format!("{:?}", err));
		return -1;
	}
	debug_message(&format!("SC.ID: {:?}", res.unwrap().id));
	0
}
