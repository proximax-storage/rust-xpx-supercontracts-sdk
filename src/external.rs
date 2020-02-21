extern "C" {
	pub fn __ping(number: usize) -> i64;
	pub fn __write_log(msg: *const u8, len: usize) -> i64;
	pub fn save_to_storage(
		file_ptr: *const u8,
		file_len: usize,
		data_ptr: *const u8,
		data_len: usize,
	) -> i64;
	pub fn save_sc_result(
		file_ptr: *const u8,
		file_len: usize,
		data_ptr: *const u8,
		data_len: usize,
	) -> i64;
	pub fn get_from_storage(
		file_ptr: *const u8,
		file_len: usize,
		data: *mut u8,
	) -> i64;
	pub fn get_http(
		url: *const u8,
		url_len: usize,
		body: *mut u8,
	) -> i64;

	pub fn flush() -> i64;
	pub fn mosaic_definition(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_account_exchange_info(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_exchange_offer_by_asset_id(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_mosaic_info(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_mosaic_infos(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_mosaics_names(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_transaction(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_transactions(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_transaction_status(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_transaction_statuses(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn get_transaction_effective_fee(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn address_alias(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn mosaic_alias(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn add_exchange_offer(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn exchange_offer(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn transfer(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn remove_exchange_offer(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn mosaic_supply_change(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;	
	pub fn register_root_namespace(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn register_sub_namespace(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn secret_lock(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn secret_proof(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn transfer_with_namespace(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn modify_metadata_address(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn modify_metadata_mosaic(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
	pub fn modify_metadata_namespace(
		data_ptr: *const u8,
		data_len: usize,
		transaction: *mut u8,
	) -> i64;
}
