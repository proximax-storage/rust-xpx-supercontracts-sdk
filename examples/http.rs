/// Example for `http_get` WasmVM function.
use std::collections::HashMap;
use xpx_supercontracts_sdk::http::{http_get, HttpRequest};

#[no_mangle]
pub extern "C" fn app_main() -> i64 {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("content-type".to_string(), "text/html".to_string());
    let req = HttpRequest {
        url: "https://s.dou.ua/files/dou-200x200.png".to_string(),
        headers: headers,
    };
    let resp = http_get(&req);
    if let Err(err) = resp {
        // Return error status
        return err as i64;
    }
    // Return response body length
    resp.unwrap().len() as i64
}
