// Example for Storage functions for WasmVM.
use serde::{Deserialize, Serialize};
use xpx_supercontracts_sdk::statuses::{Error, STATUS_SUCCESS};
use xpx_supercontracts_sdk::storage::*;

#[derive(Debug, Deserialize, Serialize)]
struct MyConfig {
    id: u64,
    title: String,
}

#[no_mangle]
pub extern "C" fn main() -> i64 {
    // Save data to Storage
    let file_name = "my_config.json".to_string();
    let data = MyConfig {
        id: 1,
        title: "some title".to_string(),
    };
    let data_bytes = serde_json::to_vec(&data);
    if data_bytes.is_err() {
        return Error::SerializeJson as i64;
    }
    let data_bytes = data_bytes.unwrap();
    let status = storage_save(&file_name, &data_bytes[..]);
    if let Err(err) = status {
        return err as i64;
    }
    // Get file data from Storage
    let file_result = storage_get(&file_name);
    if let Err(err) = file_result {
        return err as i64;
    }
    // Save Supercontract results to Storage
    let file_result = save_result(&file_name, &data_bytes[..]);
    if let Err(err) = file_result {
        return err as i64;
    }
    STATUS_SUCCESS
}
