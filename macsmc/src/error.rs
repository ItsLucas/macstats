//! Error handling for the SMC library

use std::{
    array::TryFromSliceError,
    error::Error as StdError,
    fmt::{self, Display},
    num::TryFromIntError,
};

/// This crates result type
pub type Result<T> = std::result::Result<T, Error>;

/// Possible errors that can happen
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// Signals that SMC is not available and that there is no easy way to resolve this.
    /// This could be because newer versions of macOS change the SMC API in a incompatible way
    /// or SMC is just generally not available on your system.
    SmcNotAvailable,
    /// SMC is available but there are priviliges missing to query it.
    /// This error could be resolved by using `sudo` (but it isn't guaranteed to).
    InsufficientPrivileges,
    /// Forwards any other SMC error. This usually means that SMC is available, but that something
    /// was wrong with the query.
    SmcError(i32),
    /// There was an error decoding the data response. This could mean that the key is not known,
    /// or that the data for that key could not be decoded.
    DataError {
        /// The key that this operation was failing on
        key: u32,
        /// The data type that this operation would provide
        tpe: u32,
    },
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SmcNotAvailable => write!(f, "SMC is not available, are you running on a Mac?"),
            Error::InsufficientPrivileges => {
                write!(f, "Could not perform SMC operation, try running with sudo")
            }
            Error::SmcError(code) => write!(f, "Could not perform SMC operation: {:08x}", code),
            Error::DataError { key, tpe } => write!(
                f,
                "Could not read data for key {} of type {}",
                tpe_name(key),
                tpe_name(tpe)
            ),
        }
    }
}

fn tpe_name(tpe: &u32) -> String {
    let bytes = tpe.to_be_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

pub(crate) type InternalResult<T> = std::result::Result<T, InternalError>;

#[derive(Debug)]
pub(crate) enum InternalError {
    SmcNotFound,
    SmcFailedToOpen(i32),
    NotPrivlileged,
    UnknownSmc(i32, u8),
    _UnknownKey,
    _DataKeyError(u32),
    _DataValueError,
    // for pub error
    DataError { key: u32, tpe: u32 },
}

impl From<TryFromSliceError> for InternalError {
    fn from(_: TryFromSliceError) -> Self {
        Self::_DataValueError
    }
}

impl From<TryFromIntError> for InternalError {
    fn from(_: TryFromIntError) -> Self {
        Self::_DataValueError
    }
}

impl From<InternalError> for Error {
    fn from(ie: InternalError) -> Self {
        match ie {
            InternalError::SmcNotFound => Error::SmcNotAvailable,
            InternalError::SmcFailedToOpen(_) => Error::SmcNotAvailable,
            InternalError::NotPrivlileged => Error::InsufficientPrivileges,
            InternalError::UnknownSmc(code, _) => Error::SmcError(code),
            InternalError::DataError { key, tpe } => Error::DataError { key, tpe },
            InternalError::_UnknownKey => unreachable!(),
            InternalError::_DataValueError => unreachable!(),
            InternalError::_DataKeyError(_) => unreachable!(),
        }
    }
}