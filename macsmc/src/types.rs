//! Data types and structures for SMC values

use std::{ops::Deref, time::Duration};

/// Temperature in Celsius (centigrade) scale.
/// This is the default scale being used.
///
/// # Examples
/// ```
/// # use macsmc::{Celsius, Fahrenheit};
/// let celsius = Celsius(42.0);
///
/// assert_eq!(*celsius, 42.0);
/// assert_eq!(Into::<Fahrenheit>::into(celsius), Fahrenheit(107.6));
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl Deref for Celsius {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Celsius {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

/// Temperature in Fahrenheit scale.
/// To convert from Celsius to Fahrenheit:
///
/// ```
/// # use macsmc::{Celsius, Fahrenheit};
/// let celsius = Celsius(42.0);
/// let fahrenheit = Fahrenheit::from(celsius);
///
/// assert_eq!(fahrenheit, Fahrenheit(107.6));
/// assert_eq!(*fahrenheit, 107.6);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f32);

impl Deref for Fahrenheit {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(v: Celsius) -> Self {
        Self((v.0 * (9.0 / 5.0)) + 32.0)
    }
}

impl Celsius {
    const THRESHOLDS: [Self; 4] = [Self(50.0), Self(68.0), Self(80.0), Self(90.0)];

    /// Thresholds that might be sensible to partition a temperature value
    /// into one of 4 buckets.
    ///
    /// # Examples
    /// ```
    /// # use macsmc::Celsius;
    /// let very_hot = Celsius::thresholds()[3];
    /// let quite_hot = Celsius::thresholds()[2];
    /// let warm = Celsius::thresholds()[1];
    /// let ok = Celsius::thresholds()[0];
    /// ```
    pub const fn thresholds() -> [Self; 4] {
        Self::THRESHOLDS
    }
}

/// Combination of various CPU Temperatures
/// If a sensor is missing, the value is 0.0
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CpuTemperatures {
    /// Temperature in CPU proximity. This is usually _the_ temperature, that would be shown for the CPU.
    pub proximity: Celsius,
    /// Temperature directly on the CPU Die. This is usually hotter than the proximity temperature.
    pub die: Celsius,
    /// Temperature of the integrated graphics unit of the CPU.
    /// Can be missing if there is no integrated CPU graphics.
    pub graphics: Celsius,
    /// Temperature of the uncore unit of the CPU.
    pub system_agent: Celsius,
}

/// Combination of various CPU Temperatures
/// If a sensor is missing, the value is 0.0
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct GpuTemperatures {
    /// Temperature in GPU proximity. This is usually _the_ temperature, that would be shown for the GPU.
    /// Can be missing if there is no dedicated GPU.
    pub proximity: Celsius,
    /// Temperature directly on the GPU Die. This is usually hotter than the proximity temperature.
    pub die: Celsius,
}

/// Various other CPU temperatures.
/// This list is not exhaustive nor are the sensors commonly available.
/// If a sensor is missing, the value is 0.0
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct OtherTemperatures {
    /// Memory Bank
    pub memory_bank_proximity: Celsius,
    /// Mainboard
    pub mainboard_proximity: Celsius,
    /// Platform Controller Hub
    pub platform_controller_hub_die: Celsius,
    /// Airport Proximity
    pub airport: Celsius,
    /// Left Airflow
    pub airflow_left: Celsius,
    /// Right Airflow
    pub airflow_right: Celsius,
    /// Left Thunderbolt ports
    pub thunderbolt_left: Celsius,
    /// Right Thunderbolt ports
    pub thunderbolt_right: Celsius,
    /// Heatpipe or Heatsink Sensor 1
    pub heatpipe_1: Celsius,
    /// Heatpipe or Heatsink Sensor 2
    pub heatpipe_2: Celsius,
    /// Palm rest Sensor 1
    pub palm_rest_1: Celsius,
    /// Palm rest Sensor 2
    pub palm_rest_2: Celsius,
}

/// Unit for fan speed (RPM = Revolutions per minute)
///
/// # Examples
/// ```
/// # use macsmc::Rpm;
/// let rpm = Rpm(2500.0);
/// assert_eq!(*rpm, 2500.0);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Rpm(pub f32);

impl Deref for Rpm {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Rpm {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

/// Collection of various speeds about a single fan.
/// If a sensor is missing, the value is 0.0
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FanSpeed {
    /// The current, actual, speed.
    pub actual: Rpm,
    /// The slowest that the fan can get.
    pub min: Rpm,
    /// The fastest that the fan can get.
    pub max: Rpm,
    /// The current target speed. How fast the fan should ideally be.
    pub target: Rpm,
    /// The slowest speed at which the fan is safe to operate.
    /// An value of 0.0 means that there is no sensor readout for this value,
    /// not that the fan could be turned off.
    pub safe: Rpm,
    /// How the fan is currently operating.
    pub mode: FanMode,
}

impl FanSpeed {
    /// The current speed represented as percentage of its max speed.
    /// The value is between 0.0 and 100.0
    ///
    /// # Examples
    /// ```
    /// # use macsmc::{FanSpeed, Rpm};
    /// let fan_speed = FanSpeed {
    ///     actual: Rpm(1000.0),
    ///     max: Rpm(5000.0),
    ///     ..FanSpeed::default()
    /// };
    ///
    /// assert_eq!(fan_speed.percentage(), 20.0);
    /// ```
    pub fn percentage(&self) -> f32 {
        let rpm = (*self.actual - *self.min).max(0.0);
        let pct = rpm / (*self.max - *self.min);
        100.0 * pct
    }

    /// Speed threshold for this fan.
    /// This divides the [min, max] range into 3 equally sized segments.
    ///
    /// # Examples
    /// ```
    /// # use macsmc::{FanSpeed, Rpm};
    /// let fan_speed = FanSpeed {
    ///     min: Rpm(1000.0),
    ///     max: Rpm(4000.0),
    ///     ..FanSpeed::default()
    /// };
    ///
    /// assert_eq!(fan_speed.thresholds(), [Rpm(1000.0), Rpm(2000.0), Rpm(3000.0), Rpm(4000.0)]);
    /// ```
    pub fn thresholds(&self) -> [Rpm; 4] {
        let span = (*self.max - *self.min) / 3.0;
        [
            self.min,
            Rpm(*self.min + span),
            Rpm(*self.min + (2.0 * span)),
            self.max,
        ]
    }
}

/// How a fan is being operated.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FanMode {
    /// The fan is in manual mode, its speed is a forced setting
    Forced,
    /// The fan is in automatic mode, its speed is controlled by the OS
    Auto,
}

impl From<bool> for FanMode {
    fn from(v: bool) -> Self {
        if v {
            FanMode::Forced
        } else {
            FanMode::Auto
        }
    }
}

impl Default for FanMode {
    fn default() -> Self {
        FanMode::Auto
    }
}

/// Various information about the battery in general.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatteryInfo {
    /// `true` if the system is running on battery power
    pub battery_powered: bool,
    /// `true` if the battery is currently being charged
    pub charging: bool,
    /// `true` if the system is plugged in
    pub ac_present: bool,
    /// `true` if the battery health is generally ok
    pub health_ok: bool,
    /// The highest measured temperature sensor
    pub temperature_max: Celsius,
    /// The temperature of the first battery sensor
    pub temperature_1: Celsius,
    /// The temperature of the second battery sensor
    pub temperature_2: Celsius,
}

/// Various information about the battery in detail
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BatteryDetail {
    /// The number of charging cycles of the battery
    pub cycles: u32,
    /// The current capacity ("charge") of the battery
    pub current_capacity: MilliAmpereHours,
    /// The capacity ("charge") of the battery if it was at 100%.
    /// This is different from the intial design capacity.
    /// It naturally decreases over the lifetime of the battery,
    /// meaning that older batteries cannot hold as much charge anyumore.
    pub full_capacity: MilliAmpereHours,
    /// The Current (amperage) on this battery
    /// Named `amperage` instead of `current` to prevent confusion with "current charge".
    pub amperage: MilliAmpere,
    /// The voltage on this battery
    pub voltage: Volt,
    /// If this is a positive value, it's the power delivered of this battery.
    /// If this is a negative value, it's the rate at which this battery is being charged.
    pub power: Watt,
}

impl BatteryDetail {
    /// The current charge as a percentage. Value is between 0.0 and 100.0
    ///
    /// # Examples
    /// ```
    /// # use macsmc::{BatteryDetail, MilliAmpereHours};
    /// let battery = BatteryDetail {
    ///     current_capacity: MilliAmpereHours(1000),
    ///     full_capacity: MilliAmpereHours(5000),
    ///     ..BatteryDetail::default()
    /// };
    ///
    /// assert_eq!(battery.percentage(), 20.0);
    /// ```
    pub fn percentage(&self) -> f32 {
        (100.0 * (f64::from(*self.current_capacity) / f64::from(*self.full_capacity))) as f32
    }

    /// How much time is remaining on battery, based on the current current (amperage).
    /// This is not checking if the system is marked as being "powered by battery".
    /// This only operates based on the value of `amperage`.
    /// Returns `None` if the battery is draining.
    pub fn time_remaining(&self) -> Option<Duration> {
        if *self.amperage >= 0 {
            None
        } else {
            let hours = f64::from(*self.current_capacity) / f64::from(-*self.amperage);
            Some(Duration::from_secs_f64(3600.0 * hours))
        }
    }

    /// How long it will take to load the battery based on the current current (amperage).
    /// This is not checking if the battery is marked as "being charged".
    /// This only operates based on the value of `amperage`.
    /// Returns `None` if the battery is not charging.
    pub fn time_until_full(&self) -> Option<Duration> {
        if *self.amperage <= 0 {
            None
        } else {
            let hours =
                f64::from(*self.full_capacity - *self.current_capacity) / f64::from(*self.amperage);
            Some(Duration::from_secs_f64(3600.0 * hours))
        }
    }
}

/// Various power related values of the CPU.
/// If a sensor is missing, the value is 0.0
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CpuPower {
    /// The power consumption of the CPU core
    pub core: Watt,
    /// The power consumption of the CPUs memory unit
    pub dram: Watt,
    /// The power consumption of the CPUS graphics unit
    pub gfx: Watt,
    /// The power on the rail that the CPU is running on
    pub rail: Watt,
    /// The total power consumption of the CPU
    pub total: Watt,
}

/// Value wrapper for values that are mAh units
///
/// # Examples
/// ```
/// # use macsmc::MilliAmpereHours;
/// let mah = MilliAmpereHours(42);
/// assert_eq!(*mah, 42);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MilliAmpereHours(pub u32);

impl Deref for MilliAmpereHours {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Value wrapper for values that are mA units
///
/// # Examples
/// ```
/// # use macsmc::MilliAmpere;
/// let ma = MilliAmpere(42);
/// assert_eq!(*ma, 42);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MilliAmpere(pub i32);

impl Deref for MilliAmpere {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Value wrapper for values that are V units
///
/// # Examples
/// ```
/// # use macsmc::Volt;
/// let v = Volt(42.0);
/// assert_eq!(*v, 42.0);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Volt(pub f32);

impl Deref for Volt {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Value wrapper for values that are W units
///
/// # Examples
/// ```
/// # use macsmc::Watt;
/// let w = Watt(42.0);
/// assert_eq!(*w, 42.0);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Watt(pub f32);

impl Deref for Watt {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Watt {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

impl Watt {
    const THRESHOLDS: [Self; 4] = [Self(35.0), Self(50.0), Self(70.0), Self(85.0)];

    /// Thresholds that might be sensible to partition a power value
    /// into one of 4 buckets.
    ///
    /// # Examples
    /// ```
    /// # use macsmc::Watt;
    /// let huge = Watt::thresholds()[3];
    /// let lot = Watt::thresholds()[2];
    /// let some = Watt::thresholds()[1];
    /// let little = Watt::thresholds()[0];
    /// ```
    pub fn thresholds() -> [Self; 4] {
        Self::THRESHOLDS
    }
}

/// Raw data value from a sensor
#[derive(Clone, Debug, PartialEq)]
pub enum DataValue {
    /// true/false value
    Flag(bool),
    /// float value
    Float(f32),
    /// unsigned integer
    Int(i64),
    /// signed integer
    Uint(u64),
    /// possible a string
    Str(String),
    /// Any other type that could not be decoded, containing its bytes
    Unknown(Vec<u8>),
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::Unknown(Vec::new())
    }
}

/// Return type for a debug command. Does not interpret the data.
#[derive(Debug)]
pub struct Dbg {
    /// The key for the data
    pub key: String,
    /// An error if the data could not be fetched
    /// None if the key does not exist
    /// Some(value) for other cases
    pub value: crate::Result<Option<DataValue>>,
}

/// Return type for a debug command. Does not interpret the data.
#[derive(Debug)]
pub struct DbgKeyInfo {
    /// The key for the data
    pub key: String,
    /// The expected type of the data
    pub data_type: String,
    /// The expected number of bytes to read for the data
    pub data_size: usize,
}