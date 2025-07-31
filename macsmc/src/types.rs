//! Data types and units for SMC values

use std::{fmt, ops::Deref};

/// Temperature in Celsius
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl Deref for Celsius {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

/// Temperature in Fahrenheit
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f32);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Self((c.0 * 9.0 / 5.0) + 32.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°F", self.0)
    }
}

/// Voltage in Volts
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volt(pub f32);

impl Deref for Volt {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Volt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}V", self.0)
    }
}

/// Current in Amperes
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ampere(pub f32);

impl Deref for Ampere {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Ampere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}A", self.0)
    }
}

/// Power in Watts
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Watt(pub f32);

impl Deref for Watt {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Watt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}W", self.0)
    }
}

/// Fan speed in RPM
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub f32);

impl Deref for Rpm {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Rpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} RPM", self.0)
    }
}

/// Percentage value
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f32);

impl Deref for Percentage {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}