//! Value parsing logic for SMC data

use crate::{
    error::{InternalError, InternalResult},
    types::*,
};
use std::convert::{TryFrom, TryInto};

pub(crate) trait ValueParser: Sized {
    fn parse(val: DataValue) -> InternalResult<Self>;
}

impl ValueParser for Celsius {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Float(value) => Ok(Self(value)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Rpm {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Float(value) => Ok(Self(value)),
            DataValue::Uint(v) => Ok(Self(f32::from(u16::try_from(v)?))),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for FanMode {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Flag(bool) => Ok(bool.into()),
            DataValue::Int(value) => Ok((value != 0).into()),
            DataValue::Uint(value) => Ok((value != 0).into()),
            DataValue::Float(value) => Ok((value != 0.0).into()),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BatteryStatus {
    pub(crate) charging: bool,
    pub(crate) ac_present: bool,
    pub(crate) health_ok: bool,
}

impl ValueParser for BatteryStatus {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Uint(val) => {
                let charging = val & 0x01 == 0x01;
                let ac_present = val & 0x02 == 0x02;
                let health_ok = val & 0x40 == 0x40;
                Ok(BatteryStatus {
                    charging,
                    ac_present,
                    health_ok,
                })
            }
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for MilliAmpereHours {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Uint(v) => Ok(Self(v.try_into()?)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for MilliAmpere {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Int(v) => Ok(Self(v.try_into()?)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Watt {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Float(v) => Ok(Self(v)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Volt {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Float(v) => Ok(Self(v)),
            DataValue::Uint(v) => Ok(Self(f32::from(u16::try_from(v)?) / 1000.0)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for bool {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Flag(v) => Ok(v),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for u8 {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Uint(v) => Ok(u8::try_from(v)?),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for u32 {
    fn parse(val: DataValue) -> InternalResult<Self> {
        match val {
            DataValue::Uint(v) => Ok(u32::try_from(v)?),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for DataValue {
    fn parse(val: DataValue) -> InternalResult<Self> {
        Ok(val)
    }
}

macro_rules! int_tpe {
    ($data:ident as $narrow:ty as $wide:ty as $out:ident) => {{
        Ok(crate::DataValue::$out(<$wide>::from(
            <$narrow>::from_be_bytes($data.try_into()?),
        )))
    }};
    ($data:ident as $wide:ty as $out:ident) => {{
        Ok(crate::DataValue::$out(<$wide>::from_be_bytes(
            $data.try_into()?,
        )))
    }};
}

impl DataValue {
    pub(crate) fn convert(data: &[u8], tpe: u32) -> InternalResult<Self> {
        let tpe_str = tpe.to_be_bytes();

        match &tpe_str {
            b"flag" => return Ok(DataValue::Flag(!data.is_empty() && data[0] != 0)),
            b"flt " => return Ok(DataValue::Float(f32::from_ne_bytes(data.try_into()?))),
            b"hex_" => match data.len() {
                1 => return int_tpe!(data as u8 as u64 as Uint),
                2 => return int_tpe!(data as u16 as u64 as Uint),
                4 => return int_tpe!(data as u32 as u64 as Uint),
                8 => return int_tpe!(data as u64 as u64 as Uint),
                _ => {}
            },
            b"ch8*" => {
                let has_nul_termiantor = data.contains(&0);
                let s = if has_nul_termiantor {
                    unsafe { ::std::ffi::CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                } else {
                    let mut data = data.to_vec();
                    data.push(0);
                    unsafe { ::std::ffi::CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                };
                return Ok(DataValue::Str(s));
            }
            _ => {}
        }

        match &tpe_str[..2] {
            b"fp" => {
                // fpXY, unsigned fixed point floats, X = integer width, Y = floating width
                let i = char_to_int(tpe_str[2]);
                let f = char_to_int(tpe_str[3]);
                if i + f == 16 {
                    let unsigned = u16::from_be_bytes(data.try_into()?);
                    return decode_fp_float(f32::from(unsigned), f);
                }
            }
            b"sp" => {
                // spXY, signed fixed point floats, X = integer width, Y = floating width
                let i = char_to_int(tpe_str[2]);
                let f = char_to_int(tpe_str[3]);
                if i + f == 15 {
                    let signed = i16::from_be_bytes(data.try_into()?);
                    return decode_fp_float(f32::from(signed), f);
                }
            }
            b"ui" => match &tpe_str[2..] {
                b"8 " => return int_tpe!(data as u8 as u64 as Uint),
                b"16" => return int_tpe!(data as u16 as u64 as Uint),
                b"32" => return int_tpe!(data as u32 as u64 as Uint),
                b"64" => return int_tpe!(data as u64 as Uint),
                _ => {}
            },
            b"si" => match &tpe_str[2..] {
                b"8 " => return int_tpe!(data as i8 as i64 as Int),
                b"16" => return int_tpe!(data as i16 as i64 as Int),
                b"32" => return int_tpe!(data as i32 as i64 as Int),
                b"64" => return int_tpe!(data as i64 as Int),
                _ => {}
            },
            _ => {}
        }

        Ok(DataValue::Unknown(data.to_vec()))
    }
}

fn char_to_int(c: u8) -> u8 {
    static A: u8 = b'a';
    static F: u8 = b'f';
    static N0: u8 = b'0';
    static N9: u8 = b'9';

    if c >= A && c <= F {
        c - A + 10
    } else if c >= N0 && c <= N9 {
        c - N0
    } else {
        0
    }
}

#[inline]
fn decode_fp_float(float: f32, f: u8) -> InternalResult<DataValue> {
    Ok(DataValue::Float(float / f32::from(1_u16 << f)))
}