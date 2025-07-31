//! Platform-specific sensor definitions and detection

use std::collections::HashMap;

/// Supported Apple platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    /// Intel-based Macs
    Intel,
    /// Apple Silicon M1 family
    M1,
    /// Apple Silicon M1 Pro
    M1Pro,
    /// Apple Silicon M1 Max
    M1Max,
    /// Apple Silicon M1 Ultra
    M1Ultra,
    /// Apple Silicon M2 family
    M2,
    /// Apple Silicon M2 Pro
    M2Pro,
    /// Apple Silicon M2 Max
    M2Max,
    /// Apple Silicon M2 Ultra
    M2Ultra,
    /// Apple Silicon M3 family
    M3,
    /// Apple Silicon M3 Pro
    M3Pro,
    /// Apple Silicon M3 Max
    M3Max,
    /// Apple Silicon M3 Ultra
    M3Ultra,
    /// Apple Silicon M4 family
    M4,
    /// Apple Silicon M4 Pro
    M4Pro,
    /// Apple Silicon M4 Max
    M4Max,
    /// Apple Silicon M4 Ultra
    M4Ultra,
}

/// Platform groups for easier management
impl Platform {
    /// All platforms
    pub fn all() -> Vec<Platform> {
        vec![
            Platform::Intel,
            Platform::M1, Platform::M1Pro, Platform::M1Max, Platform::M1Ultra,
            Platform::M2, Platform::M2Pro, Platform::M2Max, Platform::M2Ultra,
            Platform::M3, Platform::M3Pro, Platform::M3Max, Platform::M3Ultra,
            Platform::M4, Platform::M4Pro, Platform::M4Max, Platform::M4Ultra,
        ]
    }

    /// All Apple Silicon platforms
    pub fn apple_silicon() -> Vec<Platform> {
        vec![
            Platform::M1, Platform::M1Pro, Platform::M1Max, Platform::M1Ultra,
            Platform::M2, Platform::M2Pro, Platform::M2Max, Platform::M2Ultra,
            Platform::M3, Platform::M3Pro, Platform::M3Max, Platform::M3Ultra,
            Platform::M4, Platform::M4Pro, Platform::M4Max, Platform::M4Ultra,
        ]
    }

    /// M1 generation platforms
    pub fn m1_gen() -> Vec<Platform> {
        vec![Platform::M1, Platform::M1Pro, Platform::M1Max, Platform::M1Ultra]
    }

    /// M2 generation platforms
    pub fn m2_gen() -> Vec<Platform> {
        vec![Platform::M2, Platform::M2Pro, Platform::M2Max, Platform::M2Ultra]
    }

    /// M3 generation platforms
    pub fn m3_gen() -> Vec<Platform> {
        vec![Platform::M3, Platform::M3Pro, Platform::M3Max, Platform::M3Ultra]
    }

    /// M4 generation platforms
    pub fn m4_gen() -> Vec<Platform> {
        vec![Platform::M4, Platform::M4Pro, Platform::M4Max, Platform::M4Ultra]
    }

    /// Check if this platform is Intel-based
    pub fn is_intel(&self) -> bool {
        matches!(self, Platform::Intel)
    }

    /// Check if this platform is Apple Silicon
    pub fn is_apple_silicon(&self) -> bool {
        !self.is_intel()
    }

    /// Get the generation number (1-4 for M-series, 0 for Intel)
    pub fn generation(&self) -> u8 {
        match self {
            Platform::Intel => 0,
            Platform::M1 | Platform::M1Pro | Platform::M1Max | Platform::M1Ultra => 1,
            Platform::M2 | Platform::M2Pro | Platform::M2Max | Platform::M2Ultra => 2,
            Platform::M3 | Platform::M3Pro | Platform::M3Max | Platform::M3Ultra => 3,
            Platform::M4 | Platform::M4Pro | Platform::M4Max | Platform::M4Ultra => 4,
        }
    }
}

/// Sensor definition with platform-specific information
#[derive(Debug, Clone)]
pub struct SensorDef {
    /// SMC key (e.g., "TC0P")
    pub key: &'static str,
    /// Human-readable name
    pub name: &'static str,
    /// Sensor group (CPU, GPU, System, Sensor)
    pub group: SensorGroup,
    /// Sensor type (Temperature, Voltage, Current, Power)
    pub sensor_type: SensorType,
    /// Platforms this sensor is available on
    pub platforms: Vec<Platform>,
    /// Whether to average multiple instances of this sensor
    pub average: bool,
}

/// Sensor group classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorGroup {
    /// CPU-related sensors
    CPU,
    /// GPU-related sensors
    GPU,
    /// System-level sensors
    System,
    /// General sensor readings
    Sensor,
}

/// Type of sensor measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorType {
    /// Temperature measurements
    Temperature,
    /// Voltage measurements
    Voltage,
    /// Current measurements
    Current,
    /// Power measurements
    Power,
}

/// Get all sensor definitions
pub fn get_sensor_definitions() -> Vec<SensorDef> {
    vec![
        // Universal temperature sensors
        SensorDef {
            key: "TC0D",
            name: "CPU diode",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "TC0F",
            name: "CPU diode filtered",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "TC0P",
            name: "CPU proximity",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "TCGC",
            name: "GPU Intel Graphics",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "TG0P",
            name: "GPU proximity",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "TGDD",
            name: "GPU AMD Radeon",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::all(),
            average: false,
        },

        // Intel-specific sensors
        SensorDef {
            key: "Th1H",
            name: "Heatpipe 1",
            group: SensorGroup::Sensor,
            sensor_type: SensorType::Temperature,
            platforms: vec![Platform::Intel],
            average: false,
        },
        SensorDef {
            key: "Th2H",
            name: "Heatpipe 2",
            group: SensorGroup::Sensor,
            sensor_type: SensorType::Temperature,
            platforms: vec![Platform::Intel],
            average: false,
        },

        // M1 generation CPU cores
        SensorDef {
            key: "Tp09",
            name: "CPU efficiency core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0T",
            name: "CPU efficiency core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp01",
            name: "CPU performance core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp05",
            name: "CPU performance core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },

        // M1 generation GPU
        SensorDef {
            key: "Tg05",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },
        SensorDef {
            key: "Tg0D",
            name: "GPU 2",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m1_gen(),
            average: true,
        },

        // M2 generation CPU cores
        SensorDef {
            key: "Tp1h",
            name: "CPU efficiency core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp1t",
            name: "CPU efficiency core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp1p",
            name: "CPU efficiency core 3",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp1l",
            name: "CPU efficiency core 4",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        
        SensorDef {
            key: "Tp01",
            name: "CPU performance core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp05",
            name: "CPU performance core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp09",
            name: "CPU performance core 3",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0D",
            name: "CPU performance core 4",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0X",
            name: "CPU performance core 5",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0b",
            name: "CPU performance core 6",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0f",
            name: "CPU performance core 7",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp0j",
            name: "CPU performance core 8",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },

        // M2 generation GPU
        SensorDef {
            key: "Tg0f",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },
        SensorDef {
            key: "Tg0j",
            name: "GPU 2",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m2_gen(),
            average: true,
        },

        // M3 generation CPU cores
        SensorDef {
            key: "Te05",
            name: "CPU efficiency core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },
        SensorDef {
            key: "Te0L",
            name: "CPU efficiency core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },
        SensorDef {
            key: "Tf04",
            name: "CPU performance core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },
        SensorDef {
            key: "Tf09",
            name: "CPU performance core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },

        // M3 generation GPU
        SensorDef {
            key: "Tf14",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },
        SensorDef {
            key: "Tf18",
            name: "GPU 2",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m3_gen(),
            average: true,
        },

        // M4 generation CPU cores
        SensorDef {
            key: "Te05",
            name: "CPU efficiency core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m4_gen(),
            average: true,
        },
        SensorDef {
            key: "Te0S",
            name: "CPU efficiency core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m4_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp01",
            name: "CPU performance core 1",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m4_gen(),
            average: true,
        },
        SensorDef {
            key: "Tp05",
            name: "CPU performance core 2",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Temperature,
            platforms: Platform::m4_gen(),
            average: true,
        },

        // M4 generation GPU - different keys for different variants
        SensorDef {
            key: "Tg0G",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: vec![Platform::M4],
            average: true,
        },
        SensorDef {
            key: "Tg1U",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Temperature,
            platforms: vec![Platform::M4Pro, Platform::M4Max, Platform::M4Ultra],
            average: true,
        },

        // Power sensors
        SensorDef {
            key: "PCPC",
            name: "CPU Package",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Power,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "PCPT",
            name: "CPU Package total",
            group: SensorGroup::CPU,
            sensor_type: SensorType::Power,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "PG0R",
            name: "GPU 1",
            group: SensorGroup::GPU,
            sensor_type: SensorType::Power,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "PDTR",
            name: "DC In",
            group: SensorGroup::Sensor,
            sensor_type: SensorType::Power,
            platforms: Platform::all(),
            average: false,
        },
        SensorDef {
            key: "PSTR",
            name: "System Total",
            group: SensorGroup::Sensor,
            sensor_type: SensorType::Power,
            platforms: Platform::all(),
            average: false,
        },

        // Add more sensors as needed...
    ]
}

/// Platform detection result
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub platform: Platform,
    pub available_sensors: Vec<SensorDef>,
}

/// Detect the current platform based on system information
pub fn detect_platform() -> crate::Result<Platform> {
    use std::process::Command;
    
    // Try to get the CPU brand string from system_profiler
    if let Ok(output) = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
    {
        if let Ok(cpu_info) = String::from_utf8(output.stdout) {
            let cpu_info = cpu_info.trim().to_lowercase();
            
            // Check for Apple Silicon chips
            if cpu_info.contains("apple m1") {
                return Ok(if cpu_info.contains("ultra") {
                    Platform::M1Ultra
                } else if cpu_info.contains("max") {
                    Platform::M1Max
                } else if cpu_info.contains("pro") {
                    Platform::M1Pro
                } else {
                    Platform::M1
                });
            } else if cpu_info.contains("apple m2") {
                return Ok(if cpu_info.contains("ultra") {
                    Platform::M2Ultra
                } else if cpu_info.contains("max") {
                    Platform::M2Max
                } else if cpu_info.contains("pro") {
                    Platform::M2Pro
                } else {
                    Platform::M2
                });
            } else if cpu_info.contains("apple m3") {
                return Ok(if cpu_info.contains("ultra") {
                    Platform::M3Ultra
                } else if cpu_info.contains("max") {
                    Platform::M3Max
                } else if cpu_info.contains("pro") {
                    Platform::M3Pro
                } else {
                    Platform::M3
                });
            } else if cpu_info.contains("apple m4") {
                return Ok(if cpu_info.contains("ultra") {
                    Platform::M4Ultra
                } else if cpu_info.contains("max") {
                    Platform::M4Max
                } else if cpu_info.contains("pro") {
                    Platform::M4Pro
                } else {
                    Platform::M4
                });
            } else if cpu_info.contains("intel") {
                return Ok(Platform::Intel);
            }
        }
    }
    
    // Fallback: try to detect by checking available SMC keys
    // This is a basic heuristic - probe for platform-specific keys
    // In a real implementation, you'd want to actually connect to SMC and probe
    
    // For now, default to M1 as a reasonable fallback for Apple Silicon
    Ok(Platform::M1)
}

/// Get sensors available for a specific platform
pub fn get_sensors_for_platform(platform: Platform) -> Vec<SensorDef> {
    get_sensor_definitions()
        .into_iter()
        .filter(|sensor| sensor.platforms.contains(&platform))
        .collect()
}

/// Create a lookup map of sensors by key for a platform
pub fn create_sensor_map(platform: Platform) -> HashMap<String, SensorDef> {
    get_sensors_for_platform(platform)
        .into_iter()
        .map(|sensor| (sensor.key.to_string(), sensor))
        .collect()
}