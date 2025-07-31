//! SMC key definitions for Apple Silicon M2 and other macOS systems

/// SMC key structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmcKey {
    /// 4-character SMC key
    pub key: &'static str,
    /// Human-readable name
    pub name: &'static str,
    /// Key category
    pub category: KeyCategory,
    /// Expected data type
    pub data_type: KeyDataType,
}

/// SMC key categories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCategory {
    /// CPU related sensors
    Cpu,
    /// GPU related sensors  
    Gpu,
    /// System sensors (memory, storage, etc.)
    System,
    /// General sensors (airflow, ambient, etc.)
    Sensor,
    /// Fan control
    Fan,
    /// Battery
    Battery,
    /// Power management
    Power,
}

/// Expected data types for SMC keys
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyDataType {
    /// Temperature values
    Temperature,
    /// Voltage values
    Voltage,
    /// Current values
    Current,
    /// Power values
    Power,
    /// Fan speed (RPM)
    FanSpeed,
    /// Boolean flags
    Flag,
    /// Generic numeric
    Numeric,
    /// String data
    String,
}

impl SmcKey {
    /// Create a new SMC key
    pub const fn new(
        key: &'static str,
        name: &'static str,
        category: KeyCategory,
        data_type: KeyDataType,
    ) -> Self {
        Self {
            key,
            name,
            category,
            data_type,
        }
    }
}

/// Convert 4-character string to u32 key
pub fn key_to_u32(key: &str) -> u32 {
    if key.len() != 4 {
        return 0;
    }
    let bytes = key.as_bytes();
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Convert u32 key back to string
pub fn u32_to_key(key: u32) -> String {
    let bytes = key.to_be_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

// M2 CPU Temperature Keys
pub const M2_CPU_EFFICIENCY_CORE_1: SmcKey = SmcKey::new("Te05", "CPU Efficiency Core", KeyCategory::Cpu, KeyDataType::Temperature);

pub const M2_CPU_PERFORMANCE_CORE_1: SmcKey = SmcKey::new("Tp01", "CPU Performance Core 1", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_2: SmcKey = SmcKey::new("Tp05", "CPU Performance Core 2", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_3: SmcKey = SmcKey::new("Tp09", "CPU Performance Core 3", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_4: SmcKey = SmcKey::new("Tp0D", "CPU Performance Core 4", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_5: SmcKey = SmcKey::new("Tp0X", "CPU Performance Core 5", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_6: SmcKey = SmcKey::new("Tp0b", "CPU Performance Core 6", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_7: SmcKey = SmcKey::new("Tp0f", "CPU Performance Core 7", KeyCategory::Cpu, KeyDataType::Temperature);
pub const M2_CPU_PERFORMANCE_CORE_8: SmcKey = SmcKey::new("Tp0j", "CPU Performance Core 8", KeyCategory::Cpu, KeyDataType::Temperature);

// M2 GPU Temperature Keys
pub const M2_GPU_1: SmcKey = SmcKey::new("Tg0f", "GPU 1", KeyCategory::Gpu, KeyDataType::Temperature);
pub const M2_GPU_2: SmcKey = SmcKey::new("Tg0j", "GPU 2", KeyCategory::Gpu, KeyDataType::Temperature);

// Universal CPU Temperature Keys
pub const CPU_DIODE: SmcKey = SmcKey::new("TC0D", "CPU Diode", KeyCategory::Cpu, KeyDataType::Temperature);
pub const CPU_DIODE_FILTERED: SmcKey = SmcKey::new("TC0F", "CPU Diode Filtered", KeyCategory::Cpu, KeyDataType::Temperature);
pub const CPU_PROXIMITY: SmcKey = SmcKey::new("TC0P", "CPU Proximity", KeyCategory::Cpu, KeyDataType::Temperature);
pub const CPU_PACKAGE: SmcKey = SmcKey::new("TCAD", "CPU Package", KeyCategory::Cpu, KeyDataType::Temperature);

// Universal GPU Temperature Keys
pub const GPU_INTEL_GRAPHICS: SmcKey = SmcKey::new("TCGC", "GPU Intel Graphics", KeyCategory::Gpu, KeyDataType::Temperature);
pub const GPU_PROXIMITY: SmcKey = SmcKey::new("TG0P", "GPU Proximity", KeyCategory::Gpu, KeyDataType::Temperature);
pub const GPU_AMD_RADEON: SmcKey = SmcKey::new("TGDD", "GPU AMD Radeon", KeyCategory::Gpu, KeyDataType::Temperature);

// System Temperature Keys
pub const AIRFLOW_LEFT: SmcKey = SmcKey::new("TaLP", "Airflow Left", KeyCategory::Sensor, KeyDataType::Temperature);
pub const AIRFLOW_RIGHT: SmcKey = SmcKey::new("TaRF", "Airflow Right", KeyCategory::Sensor, KeyDataType::Temperature);
pub const NAND_STORAGE: SmcKey = SmcKey::new("TH0x", "NAND Storage", KeyCategory::System, KeyDataType::Temperature);
pub const BATTERY_1: SmcKey = SmcKey::new("TB1T", "Battery 1", KeyCategory::System, KeyDataType::Temperature);
pub const BATTERY_2: SmcKey = SmcKey::new("TB2T", "Battery 2", KeyCategory::System, KeyDataType::Temperature);
pub const AIRPORT: SmcKey = SmcKey::new("TW0P", "Airport", KeyCategory::System, KeyDataType::Temperature);
pub const MAINBOARD: SmcKey = SmcKey::new("Tm0P", "Mainboard", KeyCategory::System, KeyDataType::Temperature);
pub const THUNDERBOLT_LEFT: SmcKey = SmcKey::new("TTLD", "Thunderbolt Left", KeyCategory::System, KeyDataType::Temperature);
pub const THUNDERBOLT_RIGHT: SmcKey = SmcKey::new("TTRD", "Thunderbolt Right", KeyCategory::System, KeyDataType::Temperature);

// Power Keys
pub const POWER_CPU_PACKAGE: SmcKey = SmcKey::new("PCPC", "CPU Package Power", KeyCategory::Power, KeyDataType::Power);
pub const POWER_CPU_TOTAL: SmcKey = SmcKey::new("PCPT", "CPU Total Power", KeyCategory::Power, KeyDataType::Power);
pub const POWER_GPU_1: SmcKey = SmcKey::new("PG0R", "GPU 1 Power", KeyCategory::Power, KeyDataType::Power);
pub const POWER_GPU_2: SmcKey = SmcKey::new("PG1R", "GPU 2 Power", KeyCategory::Power, KeyDataType::Power);
pub const POWER_SYSTEM_TOTAL: SmcKey = SmcKey::new("PSTR", "System Total Power", KeyCategory::Power, KeyDataType::Power);
pub const POWER_DC_IN: SmcKey = SmcKey::new("PDTR", "DC Input Power", KeyCategory::Power, KeyDataType::Power);

// Fan Keys
pub const FAN_COUNT: SmcKey = SmcKey::new("FNum", "Fan Count", KeyCategory::Fan, KeyDataType::Numeric);

// Battery Keys
pub const BATTERY_POWERED: SmcKey = SmcKey::new("BATP", "Battery Powered", KeyCategory::Battery, KeyDataType::Flag);

// Voltage Keys
pub const VOLTAGE_CPU_CORE: SmcKey = SmcKey::new("VCAC", "CPU Core Voltage", KeyCategory::Cpu, KeyDataType::Voltage);
pub const VOLTAGE_GPU: SmcKey = SmcKey::new("VG0C", "GPU Voltage", KeyCategory::Gpu, KeyDataType::Voltage);

// Current Keys
pub const CURRENT_CPU: SmcKey = SmcKey::new("IC0R", "CPU Current", KeyCategory::Cpu, KeyDataType::Current);
pub const CURRENT_GPU: SmcKey = SmcKey::new("IG0R", "GPU Current", KeyCategory::Gpu, KeyDataType::Current);

/// Get all M2 CPU core temperature keys
pub fn m2_cpu_temperature_keys() -> Vec<&'static SmcKey> {
    vec![
        &M2_CPU_EFFICIENCY_CORE_1,
        &M2_CPU_PERFORMANCE_CORE_1,
        &M2_CPU_PERFORMANCE_CORE_2,
        &M2_CPU_PERFORMANCE_CORE_3,
        &M2_CPU_PERFORMANCE_CORE_4,
        &M2_CPU_PERFORMANCE_CORE_5,
        &M2_CPU_PERFORMANCE_CORE_6,
        &M2_CPU_PERFORMANCE_CORE_7,
        &M2_CPU_PERFORMANCE_CORE_8,
    ]
}

/// Get all M2 GPU temperature keys
pub fn m2_gpu_temperature_keys() -> Vec<&'static SmcKey> {
    vec![&M2_GPU_1]
}

/// Get system temperature keys
pub fn system_temperature_keys() -> Vec<&'static SmcKey> {
    vec![
        &NAND_STORAGE,
        &AIRPORT,
    ]
}

/// Get power monitoring keys
pub fn power_keys() -> Vec<&'static SmcKey> {
    vec![
        &POWER_SYSTEM_TOTAL,
        &POWER_DC_IN,
    ]
}