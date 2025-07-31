//! SMC connection and communication logic

use crate::{
    cffi,
    commands::{Check, DynamicCommand, PlatformCommands},
    error::InternalError,
    parsers::BatteryStatus,
    platform::Platform,
    types::*,
    Result,
};
use std::convert::TryInto;

#[derive(Debug)]
pub(crate) struct KeyInfo {
    pub(crate) key: u32,
    pub(crate) data_type: u32,
    pub(crate) data_size: u32,
}

/// The SMC client.
/// All methods take self as a mutable reference, even though
/// it is _technically_ not required.
/// This is to make sure, that a single connection can only be used
/// by one reference at a time.
///
/// # Examples
/// ```
/// # use macsmc::*;
/// # fn main() -> Result<()> {
/// let mut smc = Smc::connect()?;
/// let cpu_temp = smc.cpu_temperature()?;
/// assert!(*cpu_temp.proximity > 0.0);
/// // will disconnect
/// drop(smc);
/// # Ok(())
/// # }
/// ```
#[cfg_attr(doc, doc(cfg(target_os = "macos")))]
#[derive(Debug)]
pub struct Smc {
    inner: cffi::SMCConnection,
    platform_commands: PlatformCommands,
}

impl Smc {
    #![cfg_attr(doc, doc(cfg(target_os = "macos")))]

    /// Creates a new connection to the SMC system.
    ///
    /// # Errors
    /// [`Error::SmcNotAvailable`] If the SMC system is not available
    pub fn connect() -> Result<Self> {
        let inner = cffi::SMCConnection::new()?;
        let platform = crate::platform::detect_platform()?;
        let platform_commands = PlatformCommands::new(platform);
        Ok(Smc { inner, platform_commands })
    }

    /// Creates a new connection with a specific platform.
    /// This allows manual platform selection when auto-detection is not sufficient.
    ///
    /// # Errors
    /// [`Error::SmcNotAvailable`] If the SMC system is not available
    pub fn connect_with_platform(platform: Platform) -> Result<Self> {
        let inner = cffi::SMCConnection::new()?;
        let platform_commands = PlatformCommands::new(platform);
        Ok(Smc { inner, platform_commands })
    }

    /// Get the detected platform
    pub fn platform(&self) -> Platform {
        self.platform_commands.platform()
    }

    /// Returns an iterator over all [FanSpeed](struct.FanSpeed.html) items available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn fans(&mut self) -> Result<crate::iterators::FanIter> {
        crate::iterators::FanIter::new(self)
    }

    pub(crate) fn number_of_fans(&mut self) -> Result<u8> {
        Ok(self.inner.read_value(crate::commands::GetNumberOfFans)?)
    }

    pub(crate) fn fan_speed(&mut self, fan: u8) -> Result<FanSpeed> {
        let actual = self.inner.read_value(crate::commands::GetActualFanSpeed(fan))?;
        let min = self.inner.read_value(crate::commands::GetMinFanSpeed(fan))?;
        let max = self.inner.read_value(crate::commands::GetMaxFanSpeed(fan))?;
        let target = self.inner.read_value(crate::commands::GetTargetFanSpeed(fan))?;
        let safe = self.inner.read_value(crate::commands::GetSafeFanSpeed(fan))?;
        let mode = self.inner.read_value(crate::commands::GetFanMode(fan))?;
        Ok(FanSpeed {
            actual,
            min,
            max,
            target,
            safe,
            mode,
        })
    }

    /// Returns the overall [`BatteryInfo`]
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn battery_info(&mut self) -> Result<BatteryInfo> {
        let BatteryStatus {
            charging,
            ac_present,
            health_ok,
        } = self.inner.read_value(crate::commands::GetBatteryInfo)?;
        let battery_powered = self.inner.read_value(crate::commands::IsBatteryPowered)?;
        let temperature_max = self.inner.read_value(crate::commands::GetBatteryTemperatureMax)?;
        let temperature_1 = self.inner.read_value(crate::commands::GetBatteryTemperature1)?;
        let temperature_2 = self.inner.read_value(crate::commands::GetBatteryTemperature2)?;
        Ok(BatteryInfo {
            battery_powered,
            charging,
            ac_present,
            health_ok,
            temperature_max,
            temperature_1,
            temperature_2,
        })
    }

    pub(crate) fn number_of_batteries(&mut self) -> Result<u8> {
        Ok(self.inner.read_value(crate::commands::GetNumberOfBatteries)?)
    }

    /// Returns an iterator over all [`BatteryDetail`] items available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn battery_details(&mut self) -> Result<crate::iterators::BatteryIter> {
        Ok(crate::iterators::BatteryIter::new(self)?)
    }

    pub(crate) fn battery_detail(&mut self, battery: u8) -> Result<BatteryDetail> {
        let cycles = self.inner.read_value(crate::commands::GetBatteryCycleCount(battery))?;
        let current_capacity = self.inner.read_value(crate::commands::GetBatteryCurrentCapacity(battery))?;
        let full_capacity = self.inner.read_value(crate::commands::GetBatteryFullCapacity(battery))?;
        let amperage = self.inner.read_value(crate::commands::GetBatteryAmperage(battery))?;
        let voltage = self.inner.read_value(crate::commands::GetBatteryVoltage(battery))?;
        let power = self.inner.read_value(crate::commands::GetBatteryPower(battery))?;
        Ok(BatteryDetail {
            cycles,
            current_capacity,
            full_capacity,
            amperage,
            voltage,
            power,
        })
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn number_of_cpus(&mut self) -> Result<u8> {
        Ok(cffi::num_cpus().min(255) as u8)
    }

    /// Returns the overall [`CpuTemperatures`] available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn cpu_temperature(&mut self) -> Result<CpuTemperatures> {
        let proximity = self.inner.read_value(crate::commands::CpuProximityTemperature)?;
        let die = self.inner.read_value(crate::commands::CpuDieTemperature)?;
        let graphics = self.inner.read_value(crate::commands::CpuGfxTemperature)?;
        let system_agent = self.inner.read_value(crate::commands::CpuSystemAgentTemperature)?;
        Ok(CpuTemperatures {
            proximity,
            die,
            graphics,
            system_agent,
        })
    }

    /// Returns an iterator over all cpu core temperatures in [`Celsius`].
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    #[cfg(any(doc, target_os = "macos"))]
    pub fn cpu_core_temps(&mut self) -> Result<crate::iterators::CpuIter> {
        Ok(crate::iterators::CpuIter::new(self)?)
    }

    /// Returns platform-specific CPU core temperatures.
    /// This method uses the optimal sensor keys for the detected platform.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn platform_cpu_core_temps(&mut self) -> Result<Vec<(String, Result<Celsius>)>> {
        let keys = self.platform_commands.cpu_core_temp_keys();
        let mut results = Vec::new();
        
        for key in keys {
            let cmd = DynamicCommand::new(&key);
            let temp_result = match self.inner.opt_read_value(cmd) {
                Ok(Some(dv)) => match dv {
                    DataValue::Float(f) => Ok(Celsius(f)),
                    _ => Err(crate::Error::DataError { key: 0, tpe: 0 }), // This should be improved
                },
                Ok(None) => Err(crate::Error::DataError { key: 0, tpe: 0 }),
                Err(e) => Err(e.into()),
            };
            
            results.push((key, temp_result));
        }
        
        Ok(results)
    }

    /// Returns platform-specific GPU temperatures.
    /// This method uses the optimal sensor keys for the detected platform.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn platform_gpu_temps(&mut self) -> Result<Vec<(String, Result<Celsius>)>> {
        let keys = self.platform_commands.gpu_temp_keys();
        let mut results = Vec::new();
        
        for key in keys {
            let cmd = DynamicCommand::new(&key);
            let temp_result = match self.inner.opt_read_value(cmd) {
                Ok(Some(dv)) => match dv {
                    DataValue::Float(f) => Ok(Celsius(f)),
                    _ => Err(crate::Error::DataError { key: 0, tpe: 0 }), // This should be improved
                },
                Ok(None) => Err(crate::Error::DataError { key: 0, tpe: 0 }),
                Err(e) => Err(e.into()),
            };
            
            results.push((key, temp_result));
        }
        
        Ok(results)
    }

    /// 

    /// Check if a specific sensor is available on this platform
    pub fn has_sensor(&self, sensor_key: &str) -> bool {
        self.platform_commands.has_sensor(sensor_key)
    }

    /// Get sensor information for a specific key
    pub fn get_sensor_info(&self, sensor_key: &str) -> Option<&crate::platform::SensorDef> {
        self.platform_commands.get_sensor(sensor_key)
    }

    /// Read a sensor value by key name
    pub fn read_sensor(&mut self, sensor_key: &str) -> Result<DataValue> {
        let cmd = DynamicCommand::new(sensor_key);
        match self.inner.opt_read_value(cmd)? {
            Some(value) => Ok(value),
            None => Err(crate::Error::DataError { key: 0, tpe: 0 }),
        }
    }

    pub(crate) fn cpu_core_temperature(&mut self, core: u8) -> Result<Celsius> {
        Ok(self.inner.read_value(crate::commands::CpuCoreTemperature(core + 1))?)
    }

    /// Returns the overall [`GpuTemperatures`] available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn gpu_temperature(&mut self) -> Result<GpuTemperatures> {
        let proximity = self.inner.read_value(crate::commands::GpuProximityTemperature)?;
        let die = self.inner.read_value(crate::commands::GpuDieTemperature)?;
        Ok(GpuTemperatures { proximity, die })
    }

    /// Returns the overall information about [`OtherTemperatures`] available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn other_temperatures(&mut self) -> Result<OtherTemperatures> {
        let memory_bank_proximity = self.inner.read_value(crate::commands::GetMemoryBankProximityTemperature)?;
        let mainboard_proximity = self.inner.read_value(crate::commands::GetMainboardProximityTemperature)?;
        let platform_controller_hub_die = self.inner.read_value(crate::commands::GetPCHDieTemperature)?;
        let airport = self.inner.read_value(crate::commands::GetAirportTemperature)?;
        let airflow_left = self.inner.read_value(crate::commands::GetAirflowLeftTemperature)?;
        let airflow_right = self.inner.read_value(crate::commands::GetAirflowRightTemperature)?;
        let thunderbolt_left = self.inner.read_value(crate::commands::GetThunderboltLeftTemperature)?;
        let thunderbolt_right = self.inner.read_value(crate::commands::GetThunderboltRightTemperature)?;
        let heatpipe_1 = self.inner.read_value(crate::commands::GetHeatpipe1Temperature)?;
        let heatpipe_2 = self.inner.read_value(crate::commands::GetHeatpipe2Temperature)?;
        let palm_rest_1 = self.inner.read_value(crate::commands::GetPalmRest1Temperature)?;
        let palm_rest_2 = self.inner.read_value(crate::commands::GetPalmRest2Temperature)?;
        Ok(OtherTemperatures {
            memory_bank_proximity,
            mainboard_proximity,
            platform_controller_hub_die,
            airport,
            airflow_left,
            airflow_right,
            thunderbolt_left,
            thunderbolt_right,
            heatpipe_1,
            heatpipe_2,
            palm_rest_1,
            palm_rest_2,
        })
    }

    /// Returns the overall [`CpuPower`] information available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn cpu_power(&mut self) -> Result<CpuPower> {
        let core = self.inner.read_value(crate::commands::CpuCorePower)?;
        let dram = self.inner.read_value(crate::commands::CpuDramPower)?;
        let gfx = self.inner.read_value(crate::commands::CpuGfxPower)?;
        let rail = self.inner.read_value(crate::commands::CpuRailPower)?;
        let total = self.inner.read_value(crate::commands::CpuTotalPower)?;
        Ok(CpuPower {
            core,
            dram,
            gfx,
            rail,
            total,
        })
    }

    /// Returns the overall `GPUPower` information in [`Watt`] available.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn gpu_power(&mut self) -> Result<Watt> {
        Ok(self.inner.read_value(crate::commands::GpuRailPower)?)
    }

    /// Returns the current amount of power being in [`Watt`] drawn from DC.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn power_dc_in(&mut self) -> Result<Watt> {
        Ok(self.inner.read_value(crate::commands::DcInPower)?)
    }

    /// Returns the overall power draw in [`Watt`] of the whole system.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn power_system_total(&mut self) -> Result<Watt> {
        Ok(self.inner.read_value(crate::commands::SystemTotalPower)?)
    }

    /// Returns the number of available keys to query.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn number_of_keys(&mut self) -> Result<u32> {
        Ok(self.inner.read_value(crate::commands::NumberOfKeys)?)
    }

    /// Returns an iterator over the available keys.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn all_keys(&mut self) -> Result<crate::iterators::KeysIter> {
        crate::iterators::KeysIter::new(self)
    }

    /// Returns an iterator over the available data points.
    ///
    /// # Errors
    /// [`Error::DataError`] If there was something wrong while getting the data
    pub fn all_data(&mut self) -> Result<crate::iterators::DataIter> {
        crate::iterators::DataIter::new(self)
    }

    pub(crate) fn key_info_by_index(&mut self, index: u32) -> Result<DbgKeyInfo> {
        let info = self.inner.key_info_by_index(index)?;
        let key = info.key.to_be_bytes();
        let key = std::str::from_utf8(&key).map_err(|_| InternalError::DataError {
            key: info.key,
            tpe: info.data_type,
        })?;
        self.key_info(key)
    }

    pub(crate) fn key_data_by_index(&mut self, index: u32) -> Result<Dbg> {
        let info = self.inner.key_info_by_index(index)?;
        let key = info.key.to_be_bytes();
        let key = std::str::from_utf8(&key).map_err(|_| InternalError::DataError {
            key: info.key,
            tpe: info.data_type,
        })?;
        Ok(self.check(key))
    }

    fn key_info(&mut self, name: &str) -> Result<DbgKeyInfo> {
        let info = self.inner.key_info(Check(name))?;
        let key = info.key.to_be_bytes();
        let tpe = info.data_type.to_be_bytes();

        Ok(DbgKeyInfo {
            key: String::from_utf8_lossy(&key).to_string(),
            data_type: String::from_utf8_lossy(&tpe).to_string(),
            data_size: info.data_size.try_into().unwrap_or(usize::max_value()),
        })
    }

    fn check(&mut self, name: &str) -> Dbg {
        let value = self.inner.opt_read_value(Check(name));
        Dbg {
            key: name.to_string(),
            value: value.map_err(crate::Error::from),
        }
    }
}