use xpx_supercontracts_sdk::utils::{constructor, init, ping};

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
pub extern "C" fn ping100constructor() -> i64 {
	let _ = constructor(|| ping(100).unwrap());
	let _ = constructor(|| ping(100).unwrap());
	ping(100).unwrap()
}
