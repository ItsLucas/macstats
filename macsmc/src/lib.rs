//! SMC client for macOS
//!
//! # Examples
//! ```
//! # use macsmc::*;
//! # fn main() -> Result<()> {
//! let mut smc = Smc::connect()?;
//! let cpu_temp = smc.cpu_temperature()?;
//! assert!(*cpu_temp.proximity > 0.0);
//! // will disconnect
//! drop(smc);
//! # Ok(())
//! # }
//! ```
//!
//! See [`Smc`] for the starting point.
#![warn(anonymous_parameters)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![cfg_attr(doc, feature(doc_cfg))]

#[cfg(all(not(target_os = "macos"), not(doc)))]
compile_error!("This crate only works on macOS");

// Internal modules
mod cffi;
mod commands;
mod connection;
mod error;
mod iterators;
mod parsers;
mod platform;
mod types;

// Re-export public API
pub use connection::Smc;
pub use error::{Error, Result};
pub use iterators::{BatteryIter, DataIter, FanIter, KeysIter};
#[cfg(any(doc, target_os = "macos"))]
pub use iterators::CpuIter;
pub use platform::{Platform, SensorDef, SensorGroup, SensorType};
pub use types::*;