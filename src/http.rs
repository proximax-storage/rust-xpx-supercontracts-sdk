//! Basic HTTP functions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::external;
use crate::statuses::{Error, MultipleFunctionResult};

/// HTTP request data
#[derive(Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    /// URL for request
    pub url: String,
    /// HTTP headers for request body
    pub headers: HashMap<String, String>,
}

/// HTTP response data
#[derive(Debug, Deserialize, Serialize)]
struct HttpResponse {
    /// HTTP statuscode
    status_code: u64,
    /// Response body
    body: String,
}

/// Send HTTPP requst with specific parameters.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::http::http_get;
/// ```
pub fn http_get(request: &HttpRequest) -> MultipleFunctionResult {
    let request_body = serde_json::to_vec(&request);
    if request_body.is_err() {
        return Err(Error::SerializeJson);
    }
    let request_body = request_body.unwrap();
    let data: &mut Vec<u8> = &mut vec![];
    return unsafe {
        let body_len =
            external::get_http(request_body.as_ptr(), request_body.len(), data.as_mut_ptr());
        let data_bytes = data.get_unchecked_mut(0..body_len as usize);
        Ok(data_bytes.to_vec())
    };
}
