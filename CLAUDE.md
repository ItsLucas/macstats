# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **macstats**, a Rust command-line utility for reading system statistics from macOS SMC (System Management Controller). The project has been **completely refactored** to use a simplified, raw SMC access approach.

**NEW ARCHITECTURE:**
- **macsmc library** (`macsmc/` subdirectory) - Simplified library with raw SMC key access
- **macstats binary** (`src/main.rs`) - Updated CLI using specific SMC keys instead of abstracted APIs

## Build Commands

### Using Cargo (Standard Rust)
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized release version  
- `cargo run` - Build and run the CLI tool
- `cargo test` - Run tests
- `cargo clippy` - Run Rust linter
- `cargo fmt` - Format code

### Using Makefile (Alternative)
- `make` or `make build` - Build release version with optimizations
- `make install` - Install to system (default: /usr/local/bin)
- `make clean` - Clean build artifacts

## NEW Library Architecture (macsmc/)

The library has been **vastly simplified** and split into focused modules:

### Core Modules:
- **`client.rs`** - Raw SMC connection and key reading
- **`data.rs`** - SMC data parsing and conversion 
- **`types.rs`** - Typed units (Celsius, Watt, Volt, etc.)
- **`keys.rs`** - SMC key definitions with Apple Silicon M2 support
- **`error.rs`** - Error handling

### Raw SMC API:
```rust
use macsmc::{connect, SmcClient};

let mut client = connect()?;
let data = client.read_key("TC0P")?; // CPU Proximity temperature
let temp = data.as_temperature()?;  // Convert to Celsius
```

### Dynamic Key Support:
- **Any 4-character SMC key** can be read dynamically
- **Automatic data type parsing** based on SMC metadata
- **Type conversion** to appropriate units (temperature, power, voltage, etc.)

### Apple Silicon M2 Keys:
The library includes comprehensive M2-specific SMC keys:
- **CPU Cores**: Efficiency cores (`Tp1h`, `Tp1t`, etc.) and Performance cores (`Tp01`, `Tp05`, etc.)
- **GPU Cores**: M2 GPU sensors (`Tg0f`, `Tg0j`)
- **System**: Airflow (`TaLP`, `TaRF`), NAND storage (`TH0x`), Battery (`TB1T`, `TB2T`)
- **Power**: CPU package (`PCPC`), GPU (`PG0R`), System total (`PSTR`)

## CLI Application Updates

The CLI now uses **specific SMC keys** with **InfluxDB integration**:

### Commands:
- `macstats` - Default view (CPU and power)
- `macstats cpu` - M2 CPU core temperatures
- `macstats gpu` - M2 GPU temperatures
- `macstats system` - System temperature sensors
- `macstats power` - Power consumption, voltage, and current
- `macstats all` - Everything
- `macstats influx` - Send metrics to InfluxDB
- `macstats config` - Configuration management
- `macstats monitor` - Continuous monitoring

### InfluxDB Integration:
```bash
# Send to InfluxDB v1 with basic auth
macstats influx --url http://localhost:8086 --username admin --password secret

# Send to InfluxDB v2 with token
macstats influx --url http://localhost:8086 --org myorg --token mytoken123

# Test connection only
macstats influx --test --url http://localhost:8086 --token mytoken123

# Add custom tags
macstats influx --tags "location=office,env=production" --token mytoken123

# Continuous monitoring (uses config file)
macstats monitor --interval 30
```

### Configuration File:
```bash
# Show example configuration
macstats config --example

# Show current configuration
macstats config --show

# Configuration file location
macstats config --path
```

## Key Changes from Original

### REMOVED Features:
- ❌ High-level abstractions (CpuTemperatures, FanSpeed structs, etc.)
- ❌ Iterator-based sensor access  
- ❌ Complex sensor grouping
- ❌ Built-in fan control functionality
- ❌ Built-in battery detail calculations

### NEW Features:
- ✅ **Raw SMC key access** - Read any SMC key directly
- ✅ **Dynamic key support** - No predefined key limitations
- ✅ **Modular architecture** - Clean separation of concerns
- ✅ **Apple Silicon focus** - M2-specific key definitions
- ✅ **Type-safe conversions** - Automatic unit parsing
- ✅ **Simplified error handling** - Clear error types
- ✅ **InfluxDB integration** - Send metrics to InfluxDB v1/v2
- ✅ **Continuous monitoring** - Background data collection
- ✅ **Configuration management** - TOML-based config files
- ✅ **Flexible credentials** - Command-line or config-based auth

## Development Notes

- **macOS only** - Uses IOKit framework via FFI
- **Privileges** - May require `sudo` for some SMC operations
- **M2 optimized** - Includes comprehensive Apple Silicon M2 keys
- **Extensible** - Easy to add new SMC keys and data types
- **Raw access** - Bypasses macOS abstractions for direct SMC communication

## SMC Key Format

All SMC keys are **4-character strings** (e.g., "TC0P", "PSTR", "Tp1h"). The library automatically:
1. Converts keys to u32 format for SMC calls
2. Reads raw data and metadata  
3. Parses data based on SMC type information
4. Provides typed conversion methods

This architecture provides maximum flexibility while maintaining type safety.