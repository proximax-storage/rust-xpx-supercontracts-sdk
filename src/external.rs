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
}
