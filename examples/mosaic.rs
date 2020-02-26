use xpx_supercontracts_sdk::{
	transactions::mosaic_definition,
	utils::init,
};

pub fn create_mosaic() {}

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
	init(create_mosaic);
	0
}
