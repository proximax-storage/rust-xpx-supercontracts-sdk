//! Basic functionality for communication with Storage
//!
//! File always located inside `/root/supercontracts/` directory.
//! So it should be relative path like: `path/to/my/file.json`

use crate::external;
use crate::statuses::{FunctionResult, MultipleFunctionResult};

/// Save data to Storage. Return result status.
/// File always located inside `/root/supercontracts/` directory.
/// So it should be relative path like: `path/to/my/file.json`
///
/// # Examples
/// ```rust,no_run
/// let data = "some_data".as_bytes();
/// use xpx_supercontracts_sdk::storage::storage_save;
/// let result_status = storage_save(&"some_file.json".to_string(), data);
/// assert_eq!(result_status.unwrap(), 0);
/// ```
pub fn storage_save(file_name: &String, data: &[u8]) -> FunctionResult {
    let file_name = file_name.as_bytes();
    return unsafe {
        let res = external::save_to_storage(
            file_name.as_ptr(),
            file_name.len(),
            data.as_ptr(),
            data.len(),
        );
        Ok(res)
    };
}

/// Read file from Storage and return file data bytes.
/// File always located inside `/root/supercontracts/` directory.
/// So it should be relative path like: `path/to/my/file.json`
/// If file not exist or empty function return empty array.
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::storage::storage_get;
/// let file_data = storage_get(&"some_file.json".to_string());
/// assert_eq!(file_data.unwrap(), vec![0]);
/// ```
pub fn storage_get(file_name: &String) -> MultipleFunctionResult {
    let file_name = file_name.as_bytes();
    let data: &mut Vec<u8> = &mut vec![];
    return unsafe {
        let msg_len =
            external::get_from_storage(file_name.as_ptr(), file_name.len(), data.as_mut_ptr());
        let data_bytes = data.get_unchecked_mut(0..msg_len as usize);
        Ok(data_bytes.to_vec())
    };
}

/// Save results of Supercotract execution to Storage.
/// Return result status.
/// When Supercontract execution finished will be aggregated
/// transaction with all saved results from current Supercontract
/// execution.
/// File always located inside `/root/supercontracts/` directory.
/// So it should be relative path like: `path/to/my/file.json`
///
/// # Examples
/// ```rust,no_run
/// use xpx_supercontracts_sdk::storage::save_result;
/// let data = "some_data".as_bytes();
/// let result_status = save_result(&"some_file.json".to_string(), data);
/// assert_eq!(result_status.unwrap(), 0);
/// ```
pub fn save_result(file_name: &String, data: &[u8]) -> FunctionResult {
    let file_name = file_name.as_bytes();
    return unsafe {
        let res = external::save_sc_result(
            file_name.as_ptr(),
            file_name.len(),
            data.as_ptr(),
            data.len(),
        );
        Ok(res)
    };
}
