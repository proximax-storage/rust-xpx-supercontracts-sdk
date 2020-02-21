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
//! ## Notes
//! Supercontract functions should be
//! external with specific attributes:
//! ```ingore
//! use xpx_supercontracts_sdk::utils::{ping, debug_message};
//!
//! #[no_mangle]
//! pub extern "C" fn app_main() -> i64
//! ```
//! External function ay `Supercontract`
//! can freely naming but with only one
//! underscore symbol `_`. It's no restriction
//! to number of external functions.
//! External function should has directive `#[no_mangle]`.
//! External function can return **only** `i64` type.
//!
//! Be careful with large amount of data,
//! it's most expensive operation
//! for `Gas` calculation.
//!
//! ## Examples
//!
//! Most simple `Supercontract` that ping `WasmVM` and
//! send debug message with result to `WasmVM`.
//!
//! ```rust,no_run
//! use xpx_supercontracts_sdk::utils::{ping, debug_message};
//!
//! #[no_mangle]
//! pub extern "C" fn app_main() -> i64 {
//!     let ping_number: usize = 99;
//!     let pong_result = ping(ping_number);
//!     let msg = format!("Supercontract Ping: {:?} and Pong: {:?}", ping_number, pong_result);
//!     debug_message(&msg);
//!     return 0;
//! }
//! ```
//!
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;

mod external;
pub mod http;
pub mod statuses;
pub mod storage;
pub mod utils;
pub mod transactions;
pub mod transactions_type;
