//! SMC command definitions and action traits

use crate::{
    error::InternalResult,
    parsers::{BatteryStatus, ValueParser},
    platform::{Platform, SensorDef},
    types::*,
};
use std::{collections::HashMap, convert::TryInto, ops::Deref};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct CommandKey(u32);

impl Deref for CommandKey {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CommandKey {
    pub(crate) fn set1(self, value: u8) -> Self {
        let value = b'0' + value;
        let mut bytes = self.0.to_be_bytes();
        bytes[1] = value;
        CommandKey(u32::from_be_bytes(bytes))
    }

    pub(crate) fn set2(self, value: u8) -> Self {
        let value = b'0' + value;
        let mut bytes = self.0.to_be_bytes();
        bytes[2] = value;
        CommandKey(u32::from_be_bytes(bytes))
    }
}

pub(crate) trait ReadAction {
    type Out: ValueParser;

    fn key(&self) -> CommandKey;

    fn parse(self, val: DataValue) -> InternalResult<Self::Out>
    where
        Self: Sized,
    {
        <Self::Out as ValueParser>::parse(val)
    }
}

pub(crate) struct Check<'a>(pub(crate) &'a str);

impl<'a> ReadAction for Check<'a> {
    type Out = DataValue;

    fn key(&self) -> CommandKey {
        let bytes = self.0.as_bytes();
        let key = u32::from_be_bytes(bytes.try_into().unwrap());
        CommandKey(key)
    }
}

pub(crate) const fn smc_key(key: &'static [u8]) -> CommandKey {
    let key = [key[0], key[1], key[2], key[3]];
    let key = u32::from_be_bytes(key);
    CommandKey(key)
}

// SMC Key definitions
pub(crate) static NUMBER_OF_KEYS: CommandKey = smc_key(b"#KEY");

pub(crate) static NUM_FANS: CommandKey = smc_key(b"FNum");
pub(crate) static FAN_MODE: CommandKey = smc_key(b"F0Md");
pub(crate) static FAN_SPEED_ACTUAL: CommandKey = smc_key(b"F0Ac");
pub(crate) static FAN_SPEED_MAX: CommandKey = smc_key(b"F0Mx");
pub(crate) static FAN_SPEED_MIN: CommandKey = smc_key(b"F0Mn");
pub(crate) static FAN_SPEED_SAFE: CommandKey = smc_key(b"F0Sf");
pub(crate) static FAN_SPEED_TARGET: CommandKey = smc_key(b"F0Tg");

pub(crate) static NUM_BATTERIES: CommandKey = smc_key(b"BNum");
pub(crate) static BATTERY_POWERED: CommandKey = smc_key(b"BATP");
pub(crate) static BATTERY_INFO: CommandKey = smc_key(b"BSIn");
pub(crate) static BATTERY_CYCLES: CommandKey = smc_key(b"B0CT");
pub(crate) static BATTERY_CURRENT_CAPACITY: CommandKey = smc_key(b"B0RM");
pub(crate) static BATTERY_FULL_CAPACITY: CommandKey = smc_key(b"B0FC");
pub(crate) static BATTERY_POWER: CommandKey = smc_key(b"B0AP");
pub(crate) static BATTERY_AMPERAGE: CommandKey = smc_key(b"B0AC");
pub(crate) static BATTERY_VOLTAGE: CommandKey = smc_key(b"B0AV");

pub(crate) static TEMP_BATTERY_MAX: CommandKey = smc_key(b"TB0T");
pub(crate) static TEMP_BATTERY_1: CommandKey = smc_key(b"TB1T");
pub(crate) static TEMP_BATTERY_2: CommandKey = smc_key(b"TB2T");

pub(crate) static TEMP_CPU_CORE: CommandKey = smc_key(b"TC0C");
pub(crate) static TEMP_CPU_DIE: CommandKey = smc_key(b"TC0F");
pub(crate) static TEMP_CPU_SYSTEM_AGENT: CommandKey = smc_key(b"TCSA");
pub(crate) static TEMP_CPU_GFX: CommandKey = smc_key(b"TCGC");
pub(crate) static TEMP_CPU_PROXIMITY: CommandKey = smc_key(b"TC0P");

pub(crate) static TEMP_GPU_PROXIMITY: CommandKey = smc_key(b"TG0P");
pub(crate) static TEMP_GPU_DIE: CommandKey = smc_key(b"TGDD");

pub(crate) static TEMP_MEM_PROXIMITY: CommandKey = smc_key(b"TM0P");
pub(crate) static TEMP_PLATFORM_CONTROLLER_HUB_DIE: CommandKey = smc_key(b"TPCD");
pub(crate) static TEMP_HEATPIPE_1: CommandKey = smc_key(b"Th1H");
pub(crate) static TEMP_HEATPIPE_2: CommandKey = smc_key(b"Th2H");
pub(crate) static TEMP_MAINBOARD_PROXIMITY: CommandKey = smc_key(b"Tm0P");

pub(crate) static TEMP_PALM_REST_1: CommandKey = smc_key(b"Ts0P");
pub(crate) static TEMP_PALM_REST_2: CommandKey = smc_key(b"Ts1P");
pub(crate) static TEMP_AIRPORT: CommandKey = smc_key(b"TW0P");
pub(crate) static TEMP_AIRFLOW_LEFT: CommandKey = smc_key(b"TaLC");
pub(crate) static TEMP_AIRFLOW_RIGHT: CommandKey = smc_key(b"TaRC");
pub(crate) static TEMP_THUNDERBOLT_LEFT: CommandKey = smc_key(b"TTLD");
pub(crate) static TEMP_THUNDERBOLT_RIGHT: CommandKey = smc_key(b"TTRD");

pub(crate) static POWER_CPU_CORE: CommandKey = smc_key(b"PCPC");
pub(crate) static POWER_CPU_DRAM: CommandKey = smc_key(b"PCPD");
pub(crate) static POWER_CPU_GFX: CommandKey = smc_key(b"PCPG");
pub(crate) static POWER_CPU_RAIL: CommandKey = smc_key(b"PC0R");
pub(crate) static POWER_CPU_TOTAL: CommandKey = smc_key(b"PCPT");
pub(crate) static POWER_DC_IN: CommandKey = smc_key(b"PDTR");
pub(crate) static POWER_GPU_RAIL: CommandKey = smc_key(b"PG0R");
pub(crate) static POWER_SYSTEM_TOTAL: CommandKey = smc_key(b"PSTR");

macro_rules! read_impl {
    ($struct:ident = $key:ident -> $out:tt) => {
        #[derive(Debug)]
        pub(crate) struct $struct;

        impl $crate::commands::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key
            }
        }
    };

    ($struct:ident($arg:tt) = $key:ident -> $out:tt) => {
        #[derive(Debug)]
        pub(crate) struct $struct(pub(crate) $arg);

        impl $crate::commands::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key.set1(self.0)
            }
        }
    };

    ($struct:ident($arg:tt) == $key:ident -> $out:tt) => {
        #[derive(Debug)]
        pub(crate) struct $struct(pub(crate) $arg);

        impl $crate::commands::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key.set2(self.0)
            }
        }
    };
}

read_impl!(NumberOfKeys = NUMBER_OF_KEYS -> u32);

read_impl!(GetNumberOfFans = NUM_FANS -> u8);
read_impl!(GetActualFanSpeed(u8) = FAN_SPEED_ACTUAL -> Rpm);
read_impl!(GetMinFanSpeed(u8) = FAN_SPEED_MIN -> Rpm);
read_impl!(GetMaxFanSpeed(u8) = FAN_SPEED_MAX -> Rpm);
read_impl!(GetTargetFanSpeed(u8) = FAN_SPEED_TARGET -> Rpm);
read_impl!(GetSafeFanSpeed(u8) = FAN_SPEED_SAFE -> Rpm);
read_impl!(GetFanMode(u8) = FAN_MODE -> FanMode);

read_impl!(GetNumberOfBatteries = NUM_BATTERIES -> u8);
read_impl!(IsBatteryPowered = BATTERY_POWERED -> bool);
read_impl!(GetBatteryInfo = BATTERY_INFO -> BatteryStatus);
read_impl!(GetBatteryCycleCount(u8) = BATTERY_CYCLES -> u32);
read_impl!(GetBatteryCurrentCapacity(u8) = BATTERY_CURRENT_CAPACITY -> MilliAmpereHours);
read_impl!(GetBatteryFullCapacity(u8) = BATTERY_FULL_CAPACITY -> MilliAmpereHours);
read_impl!(GetBatteryAmperage(u8) = BATTERY_AMPERAGE -> MilliAmpere);
read_impl!(GetBatteryVoltage(u8) = BATTERY_VOLTAGE -> Volt);
read_impl!(GetBatteryPower(u8) = BATTERY_POWER -> Watt);
read_impl!(GetBatteryTemperatureMax = TEMP_BATTERY_MAX -> Celsius);
read_impl!(GetBatteryTemperature1 = TEMP_BATTERY_1 -> Celsius);
read_impl!(GetBatteryTemperature2 = TEMP_BATTERY_2 -> Celsius);

read_impl!(CpuProximityTemperature = TEMP_CPU_PROXIMITY -> Celsius);
read_impl!(CpuDieTemperature = TEMP_CPU_DIE -> Celsius);
read_impl!(CpuGfxTemperature = TEMP_CPU_GFX -> Celsius);
read_impl!(CpuSystemAgentTemperature = TEMP_CPU_SYSTEM_AGENT -> Celsius);
read_impl!(CpuCoreTemperature(u8) == TEMP_CPU_CORE -> Celsius);

read_impl!(GpuProximityTemperature = TEMP_GPU_PROXIMITY -> Celsius);
read_impl!(GpuDieTemperature = TEMP_GPU_DIE -> Celsius);

read_impl!(GetMemoryBankProximityTemperature = TEMP_MEM_PROXIMITY -> Celsius);
read_impl!(GetMainboardProximityTemperature = TEMP_MAINBOARD_PROXIMITY -> Celsius);
read_impl!(GetPCHDieTemperature = TEMP_PLATFORM_CONTROLLER_HUB_DIE -> Celsius);
read_impl!(GetAirportTemperature = TEMP_AIRPORT -> Celsius);
read_impl!(GetAirflowLeftTemperature = TEMP_AIRFLOW_LEFT -> Celsius);
read_impl!(GetAirflowRightTemperature = TEMP_AIRFLOW_RIGHT -> Celsius);
read_impl!(GetThunderboltLeftTemperature = TEMP_THUNDERBOLT_LEFT -> Celsius);
read_impl!(GetThunderboltRightTemperature = TEMP_THUNDERBOLT_RIGHT -> Celsius);
read_impl!(GetHeatpipe1Temperature = TEMP_HEATPIPE_1 -> Celsius);
read_impl!(GetHeatpipe2Temperature = TEMP_HEATPIPE_2 -> Celsius);
read_impl!(GetPalmRest1Temperature = TEMP_PALM_REST_1 -> Celsius);
read_impl!(GetPalmRest2Temperature = TEMP_PALM_REST_2 -> Celsius);

read_impl!(CpuCorePower = POWER_CPU_CORE -> Watt);
read_impl!(CpuDramPower = POWER_CPU_DRAM -> Watt);
read_impl!(CpuGfxPower = POWER_CPU_GFX -> Watt);
read_impl!(CpuRailPower = POWER_CPU_RAIL -> Watt);
read_impl!(CpuTotalPower = POWER_CPU_TOTAL -> Watt);
read_impl!(GpuRailPower = POWER_GPU_RAIL -> Watt);
read_impl!(DcInPower = POWER_DC_IN -> Watt);
read_impl!(SystemTotalPower = POWER_SYSTEM_TOTAL -> Watt);

/// Platform-aware command resolver
#[derive(Debug)]
pub(crate) struct PlatformCommands {
    platform: Platform,
    sensor_map: HashMap<String, SensorDef>,
}

impl PlatformCommands {
    pub(crate) fn new(platform: Platform) -> Self {
        let sensor_map = crate::platform::create_sensor_map(platform);
        Self {
            platform,
            sensor_map,
        }
    }

    /// Get available CPU core temperature keys for this platform
    pub(crate) fn cpu_core_temp_keys(&self) -> Vec<String> {
        self.sensor_map
            .values()
            .filter(|sensor| {
                sensor.group == crate::platform::SensorGroup::CPU
                    && sensor.sensor_type == crate::platform::SensorType::Temperature
                    && sensor.average
            })
            .map(|sensor| sensor.key.to_string())
            .collect()
    }

    /// Get available GPU temperature keys for this platform
    pub(crate) fn gpu_temp_keys(&self) -> Vec<String> {
        self.sensor_map
            .values()
            .filter(|sensor| {
                sensor.group == crate::platform::SensorGroup::GPU
                    && sensor.sensor_type == crate::platform::SensorType::Temperature
            })
            .map(|sensor| sensor.key.to_string())
            .collect()
    }

    /// Get the best CPU proximity temperature key for this platform
    pub(crate) fn cpu_proximity_key(&self) -> Option<CommandKey> {
        // Try platform-specific keys first, then fall back to universal
        let fallback_keys = vec!["TC0P", "TCAD", "TC0D"];
        
        for key in fallback_keys {
            if self.sensor_map.contains_key(key) {
                return Some(smc_key(key.as_bytes()));
            }
        }
        
        // Default fallback
        Some(TEMP_CPU_PROXIMITY)
    }

    /// Get the best GPU proximity temperature key for this platform
    pub(crate) fn gpu_proximity_key(&self) -> Option<CommandKey> {
        let fallback_keys = vec!["TG0P", "TGDD", "TCGC"];
        
        for key in fallback_keys {
            if self.sensor_map.contains_key(key) {
                return Some(smc_key(key.as_bytes()));
            }
        }
        
        // Default fallback
        Some(TEMP_GPU_PROXIMITY)
    }

    /// Check if a sensor key is available on this platform
    pub(crate) fn has_sensor(&self, key: &str) -> bool {
        self.sensor_map.contains_key(key)
    }

    /// Get sensor definition by key
    pub(crate) fn get_sensor(&self, key: &str) -> Option<&SensorDef> {
        self.sensor_map.get(key)
    }

    /// Get platform
    pub(crate) fn platform(&self) -> Platform {
        self.platform
    }
}

/// Dynamic SMC command for platform-specific sensors
pub(crate) struct DynamicCommand {
    key: CommandKey,
}

impl DynamicCommand {
    pub(crate) fn new(key_str: &str) -> Self {
        println!("Creating DynamicCommand for key: {}", key_str);
        let key = if key_str.len() == 4 {
            let key_bytes = key_str.as_bytes();
            if key_bytes.len() >= 4 {
                let key_array = [key_bytes[0], key_bytes[1], key_bytes[2], key_bytes[3]];
                CommandKey(u32::from_be_bytes(key_array))
            } else {
                smc_key(b"????")
            }
        } else {
            // Handle dynamic keys with placeholders
            smc_key(b"????") // This should be handled more carefully in practice
        };
        
        Self { key }
    }
}

impl ReadAction for DynamicCommand {
    type Out = DataValue;

    fn key(&self) -> CommandKey {
        self.key
    }
}