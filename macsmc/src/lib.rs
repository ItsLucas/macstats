//! Simplified SMC client for macOS
//! 
//! Provides raw SMC key reading with dynamic key support and typed data parsing.

#![warn(missing_docs)]
#![cfg_attr(doc, feature(doc_cfg))]

#[cfg(all(not(target_os = "macos"), not(doc)))]
compile_error!("This crate only works on macOS");

pub mod client;
pub mod data;
pub mod error;
pub mod keys;
pub mod types;

pub use client::SmcClient;
pub use data::SmcData;
pub use error::{Result, SmcError};
pub use types::*;

/// Create a new SMC client connection
pub fn connect() -> Result<SmcClient> {
    SmcClient::new()
}