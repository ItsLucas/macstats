//! Error types for SMC operations

use std::{error::Error as StdError, fmt};

/// Result type for SMC operations
pub type Result<T> = std::result::Result<T, SmcError>;

/// SMC operation errors
#[derive(Debug, Clone)]
pub enum SmcError {
    /// SMC is not available on this system
    NotAvailable,
    /// Insufficient privileges to access SMC
    InsufficientPrivileges,
    /// SMC returned an error code
    SmcError(i32),
    /// Invalid SMC key format
    InvalidKey(String),
    /// Data parsing error
    DataError { key: String, data_type: String },
    /// Unknown data type
    UnknownDataType(String),
}

impl fmt::Display for SmcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmcError::NotAvailable => write!(f, "SMC is not available"),
            SmcError::InsufficientPrivileges => write!(f, "Insufficient privileges to access SMC"),
            SmcError::SmcError(code) => write!(f, "SMC error: {:#x}", code),
            SmcError::InvalidKey(key) => write!(f, "Invalid SMC key: {}", key),
            SmcError::DataError { key, data_type } => {
                write!(f, "Data error for key {}: type {}", key, data_type)
            }
            SmcError::UnknownDataType(data_type) => {
                write!(f, "Unknown data type: {}", data_type)
            }
        }
    }
}

impl StdError for SmcError {}