//! # XPX Supercontracts SDK
//! The documentation describe basic functions and tools
//! for implementation `XPX Supercontracts` based on
//! `XPX Wasm VM` (WebAssembly Virtual Machine).
//!
//! Currently supported target: `wasm32-unknown-unknown`.
//!
//! ## Development flow
//! * Check is installed target: `rustup target list | grep wasm32-unknown-unknown`
//! * If target not install add it: ``rustup target add wasm32-unknown-unknown
//! * * Create boilerplate supercontract app: `cargo new my-supercontract`
//! * Add to `Cargo.toml`:
//! ```ignore
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! xpx_supercontracts_sdk = "0.1"
//! ```
//! * Here we go! Application ready to build. To build application: `cargo build --release --target wasm32-unknown-unknown`.
//!
//! ## Examples
//!
//! Most simple supercontract that ping VM and
//! send debug message with result to VM.
//!
//! ```rust,no_run
//! use xpx_supercontracts_sdk::{ping, debug_message};
//!
//! #[no_mangle]
//! pub extern "C" fn app_main() -> i64 {
//!     let ping_number: usize = 99;
//!     let pong_result = ping(ping_number);
//!     let msg = format!("Supercontract Ping: {:?} and Pong: {:?}", ping_number, pong_result);
//!     debug_message(msg);
//!     return 0;
//! }
//! ```
//!
extern crate serde;
extern crate serde_json;

mod external;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct HttpRequest {
    url: String,
    headers: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HttpResponse {
    status_code: u64,
    body: String,
}

/// Send ping message to `WasmVM`. Successful result should be
/// incremented value.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::ping;
/// let respond = ping(10);
/// assert_eq!(respond, 11);
/// ```
pub fn ping(msg: usize) -> i64 {
    return unsafe { external::__ping(msg) };
}

/// Send debug message to `WasmVM`. It's convenient
/// basic function for development debugging.
/// Message that was sent will display in `WasmVM`
/// stdout as information log message. It not affect
/// basic Supercontract execution but should be
/// removed from `release` version, because it
/// will spend `Gas` (unit ticks).
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::debug_message;
/// debug_message("Debug message from Supercontract".to_string());
/// ```
pub fn debug_message(msg: String) {
    let raw_msg = msg.as_bytes();
    unsafe {
        external::__write_log(raw_msg.as_ptr(), raw_msg.len());
    };
}

pub fn storage_save(file_name: String, data: &[u8]) -> i64 {
    let file_name = file_name.as_bytes();
    return unsafe {
        external::save_to_storage(
            file_name.as_ptr(),
            file_name.len(),
            data.as_ptr(),
            data.len(),
        )
    };
}

pub fn save_result(file_name: String, data: &[u8]) -> i64 {
    let file_name = file_name.as_bytes();
    return unsafe {
        external::save_sc_result(
            file_name.as_ptr(),
            file_name.len(),
            data.as_ptr(),
            data.len(),
        )
    };
}

/// Read file from Storage and return file data bytes.
/// File always located inside `/root/supercontracts/` directory.
/// So it should be relative path like: `path/to/my/file.json`
/// If file not exist or empty function return empty array.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::storage_get;
/// let file_data = storage_get("some_file.json".to_string());
/// ```
pub fn storage_get(file_name: String) -> Vec<u8> {
    let file_name = file_name.as_bytes();
    let data: &mut Vec<u8> = &mut vec![];
    return unsafe {
        let msg_len =
            external::get_from_storage(file_name.as_ptr(), file_name.len(), data.as_mut_ptr());
        let data_bytes = data.get_unchecked_mut(0..msg_len as usize);
        data_bytes.to_vec()
    };
}
/*
#[no_mangle]
pub extern "C" fn http_get() -> i64 {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("content-type".to_string(), "text/html".to_string());
    let request = HttpRequest {
        url: "http://google.com/".to_string(),
        headers: headers,
    };
    let request_body = serde_json::to_vec(&request);
    if request_body.is_err() {
        return -1;
    }
    let request_body = request_body.unwrap();
    let body: &mut Vec<u8> = &mut vec![];
    let res = unsafe {
        let body_len = sys::get_http(request_body.as_ptr(), request_body.len(), body.as_mut_ptr());
        //let _resp = HttpResponse::decode(&body);
        //sys::__write_log(body.as_ptr(), body_len as usize);
        body_len
    };
    return res;
}

*/
