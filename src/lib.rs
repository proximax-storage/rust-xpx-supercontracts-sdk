extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub mod sys {
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
        pub fn get_from_storage(file_ptr: *const u8, file_len: usize, data: *mut u8) -> i64;
        pub fn get_http(url: *const u8, url_len: usize, body: *mut u8) -> i64;
    }
}

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


#[no_mangle]
pub extern "C" fn ping() -> i64 {
    let res = unsafe { sys::__ping(99) };
    return res;
}

#[no_mangle]
pub extern "C" fn log_write() -> i64 {
    let msg = format!("[LOG] Test message");
    let msg = msg.as_bytes();
    unsafe {
        sys::__write_log(msg.as_ptr(), msg.len());
    };
    return msg.len() as i64;
}

#[no_mangle]
pub extern "C" fn storage_save() -> i64 {
    let file_name = "test_file".as_bytes();
    let file_data = "some data".as_bytes();
    let res = unsafe {
        sys::save_to_storage(
            file_name.as_ptr(),
            file_name.len(),
            file_data.as_ptr(),
            file_data.len(),
        )
    };
    return res;
}

#[no_mangle]
pub extern "C" fn save_result() -> i64 {
    let file_name = "test_file".as_bytes();
    let file_data = "some data".as_bytes();
    let res = unsafe {
        sys::save_sc_result(
            file_name.as_ptr(),
            file_name.len(),
            file_data.as_ptr(),
            file_data.len(),
        )
    };
    return res;
}

#[no_mangle]
pub extern "C" fn storage_get() -> i64 {
    let file_name = "test_file".as_bytes();
    let data: &mut Vec<u8> = &mut vec![];
    let res = unsafe {
        let msg_len = sys::get_from_storage(file_name.as_ptr(), file_name.len(), data.as_mut_ptr());
        let data_bytes = data.get_unchecked_mut(0..msg_len as usize);
        let data_str = std::str::from_utf8(&data_bytes);
        let msg = format!("RES: {:?} | LEN: {:?}", data_str, msg_len);
        sys::__write_log(msg.as_bytes().as_ptr(), msg.as_bytes().len());
        msg_len
    };
    if res < 0 {
        return -1;
    }
    return res as i64;
}

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

