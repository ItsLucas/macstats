//! SMC data parsing and conversion

use crate::{error::*, types::*};
use std::{convert::TryInto, ffi::CStr};

/// Raw SMC data value
#[derive(Debug, Clone)]
pub enum SmcData {
    /// Boolean flag
    Flag(bool),
    /// 32-bit float
    Float(f32),
    /// Signed integer
    Int(i64),
    /// Unsigned integer
    Uint(u64),
    /// String value
    String(String),
    /// Unknown/raw bytes
    Raw(Vec<u8>),
}

impl SmcData {
    /// Parse raw SMC data based on type
    pub fn parse(data: &[u8], data_type: &str) -> Result<Self> {
        match data_type {
            "flag" => Ok(SmcData::Flag(!data.is_empty() && data[0] != 0)),
            "flt " => {
                if data.len() >= 4 {
                    let bytes: [u8; 4] = data[..4].try_into().map_err(|_| {
                        SmcError::DataError {
                            key: "unknown".to_string(),
                            data_type: data_type.to_string(),
                        }
                    })?;
                    // SMC floats are stored in little-endian format on modern Macs
                    Ok(SmcData::Float(f32::from_le_bytes(bytes)))
                } else {
                    Err(SmcError::DataError {
                        key: "unknown".to_string(),
                        data_type: data_type.to_string(),
                    })
                }
            }
            "ch8*" => {
                let s = if data.contains(&0) {
                    unsafe { CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                } else {
                    let mut data = data.to_vec();
                    data.push(0);
                    unsafe { CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                };
                Ok(SmcData::String(s))
            }
            t if t.starts_with("ui") => {
                match &t[2..] {
                    "8 " if data.len() >= 1 => Ok(SmcData::Uint(data[0] as u64)),
                    "16" if data.len() >= 2 => {
                        let bytes: [u8; 2] = data[..2].try_into().unwrap();
                        Ok(SmcData::Uint(u16::from_be_bytes(bytes) as u64))
                    }
                    "32" if data.len() >= 4 => {
                        let bytes: [u8; 4] = data[..4].try_into().unwrap();
                        Ok(SmcData::Uint(u32::from_be_bytes(bytes) as u64))
                    }
                    "64" if data.len() >= 8 => {
                        let bytes: [u8; 8] = data[..8].try_into().unwrap();
                        Ok(SmcData::Uint(u64::from_be_bytes(bytes)))
                    }
                    _ => Ok(SmcData::Raw(data.to_vec())),
                }
            }
            t if t.starts_with("si") => {
                match &t[2..] {
                    "8 " if data.len() >= 1 => Ok(SmcData::Int(data[0] as i8 as i64)),
                    "16" if data.len() >= 2 => {
                        let bytes: [u8; 2] = data[..2].try_into().unwrap();
                        Ok(SmcData::Int(i16::from_be_bytes(bytes) as i64))
                    }
                    "32" if data.len() >= 4 => {
                        let bytes: [u8; 4] = data[..4].try_into().unwrap();
                        Ok(SmcData::Int(i32::from_be_bytes(bytes) as i64))
                    }
                    "64" if data.len() >= 8 => {
                        let bytes: [u8; 8] = data[..8].try_into().unwrap();
                        Ok(SmcData::Int(i64::from_be_bytes(bytes)))
                    }
                    _ => Ok(SmcData::Raw(data.to_vec())),
                }
            }
            t if t.starts_with("fp") => {
                // fpXY: unsigned fixed point, X = integer bits, Y = fractional bits
                if data.len() >= 2 && t.len() == 4 {
                    let f_bits = (t.chars().nth(3).unwrap() as u8 - b'0') as u32;
                    let bytes: [u8; 2] = data[..2].try_into().unwrap();
                    let raw = u16::from_be_bytes(bytes) as f32;
                    let value = raw / (1 << f_bits) as f32;
                    Ok(SmcData::Float(value))
                } else {
                    Ok(SmcData::Raw(data.to_vec()))
                }
            }
            t if t.starts_with("sp") => {
                // spXY: signed fixed point
                if data.len() >= 2 && t.len() == 4 {
                    let f_bits = (t.chars().nth(3).unwrap() as u8 - b'0') as u32;
                    let bytes: [u8; 2] = data[..2].try_into().unwrap();
                    let raw = i16::from_be_bytes(bytes) as f32;
                    let value = raw / (1 << f_bits) as f32;
                    Ok(SmcData::Float(value))
                } else {
                    Ok(SmcData::Raw(data.to_vec()))
                }
            }
            _ => Ok(SmcData::Raw(data.to_vec())),
        }
    }

    /// Convert to temperature (Celsius)
    pub fn as_temperature(&self) -> Result<Celsius> {
        match self {
            SmcData::Float(f) => Ok(Celsius(*f)),
            SmcData::Int(i) => Ok(Celsius(*i as f32)),
            SmcData::Uint(u) => Ok(Celsius(*u as f32)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "temperature".to_string(),
            }),
        }
    }

    /// Convert to voltage
    pub fn as_voltage(&self) -> Result<Volt> {
        match self {
            SmcData::Float(f) => Ok(Volt(*f)),
            SmcData::Int(i) => Ok(Volt(*i as f32 / 1000.0)), // millivolts to volts
            SmcData::Uint(u) => Ok(Volt(*u as f32 / 1000.0)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "voltage".to_string(),
            }),
        }
    }

    /// Convert to current (Amperes)
    pub fn as_current(&self) -> Result<Ampere> {
        match self {
            SmcData::Float(f) => Ok(Ampere(*f)),
            SmcData::Int(i) => Ok(Ampere(*i as f32 / 1000.0)), // milliamps to amps
            SmcData::Uint(u) => Ok(Ampere(*u as f32 / 1000.0)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "current".to_string(),
            }),
        }
    }

    /// Convert to power (Watts)
    pub fn as_power(&self) -> Result<Watt> {
        match self {
            SmcData::Float(f) => Ok(Watt(*f)),
            SmcData::Int(i) => Ok(Watt(*i as f32)),
            SmcData::Uint(u) => Ok(Watt(*u as f32)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "power".to_string(),
            }),
        }
    }

    /// Convert to RPM
    pub fn as_rpm(&self) -> Result<Rpm> {
        match self {
            SmcData::Float(f) => Ok(Rpm(*f)),
            SmcData::Int(i) => Ok(Rpm(*i as f32)),
            SmcData::Uint(u) => Ok(Rpm(*u as f32)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "rpm".to_string(),
            }),
        }
    }

    /// Convert to percentage
    pub fn as_percentage(&self) -> Result<Percentage> {
        match self {
            SmcData::Float(f) => Ok(Percentage(*f)),
            SmcData::Int(i) => Ok(Percentage(*i as f32)),
            SmcData::Uint(u) => Ok(Percentage(*u as f32)),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "percentage".to_string(),
            }),
        }
    }

    /// Convert to boolean
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            SmcData::Flag(b) => Ok(*b),
            SmcData::Int(i) => Ok(*i != 0),
            SmcData::Uint(u) => Ok(*u != 0),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "bool".to_string(),
            }),
        }
    }

    /// Get raw integer value
    pub fn as_u64(&self) -> Result<u64> {
        match self {
            SmcData::Uint(u) => Ok(*u),
            SmcData::Int(i) => Ok(*i as u64),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "u64".to_string(),
            }),
        }
    }

    /// Get raw string value
    pub fn as_string(&self) -> Result<String> {
        match self {
            SmcData::String(s) => Ok(s.clone()),
            _ => Err(SmcError::DataError {
                key: "unknown".to_string(),
                data_type: "string".to_string(),
            }),
        }
    }
}