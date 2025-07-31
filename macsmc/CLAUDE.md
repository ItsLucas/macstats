# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace containing two main components:
- **macsmc**: A library crate that interfaces with macOS SMC (System Management Controller) to read hardware sensors
- **macstats**: A command-line utility built on top of the macsmc library

The project specifically targets macOS and provides access to system hardware information like CPU/GPU temperatures, fan speeds, battery status, and power consumption through Apple's SMC interface.

## Common Development Commands

### Building
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized release version
- `make` or `make build` - Build release version using Makefile
- `cargo build --bin macstats` - Build just the CLI tool

### Testing and Quality
- `cargo test` - Run all tests
- `cargo clippy` - Run linter
- `cargo fmt` - Format code
- `cargo doc` - Generate documentation

### Installation
- `make install` - Install to `/usr/local/bin/` (use `PREFIX=/path make install` for custom location)
- `cargo install --path .` - Install using cargo

### Clean
- `cargo clean` - Clean build artifacts
- `make clean` - Same as cargo clean

## Architecture

### Core Library (macsmc/)
- **lib.rs:1854** - Main library implementation with comprehensive SMC interface
- Provides safe Rust wrappers around macOS SMC C APIs
- Key structures:
  - `Smc` - Main connection struct for SMC communication
  - Temperature types: `Celsius`, `Fahrenheit`, `CpuTemperatures`, `GpuTemperatures`, `OtherTemperatures`
  - Power types: `Watt`, `CpuPower`
  - Fan types: `Rpm`, `FanSpeed`, `FanMode`
  - Battery types: `BatteryInfo`, `BatteryDetail`, `MilliAmpereHours`, `MilliAmpere`, `Volt`
- Iterators for accessing multiple sensors: `FanIter`, `BatteryIter`, `CpuIter`
- Debug functionality for raw SMC key access

### CLI Application (src/main.rs)
- Command-line interface with filtering options:
  - `temp`/`temps` - All temperature sensors
  - `cpu` - CPU temperatures only
  - `gpu` - GPU temperatures only  
  - `other` - Other temperature sensors
  - `fan`/`fans` - Fan speeds
  - `battery` - Battery information
  - `power` - Power consumption
  - `debug` - Raw SMC data dump
  - `all` - Everything except debug
- Default behavior shows CPU, fan, battery, and power info
- Colored output with visual indicators (sparklines)

### Platform Constraints
- **macOS only** - Code includes `compile_error!` for non-macOS targets
- Requires system privileges for SMC access
- Uses IOKit framework for low-level hardware communication

### Key Implementation Details
- SMC communication through `cffi` module with direct IOKit bindings
- Comprehensive error handling with specific error types for SMC failures
- Fixed-point floating point decoding for sensor values
- Support for various SMC data types (flags, floats, integers, strings)
- Temperature thresholds and visual feedback system
- **Platform-aware sensor access**: Automatic detection and support for different Apple platforms (Intel, M1-M4 generations)

### Platform Support
The library now includes comprehensive platform detection and sensor mapping:
- **Platform Detection**: Automatic detection via `sysctl machdep.cpu.brand_string`
- **Platform-Specific Sensors**: Different sensor keys based on hardware (Intel vs Apple Silicon)
- **Multi-Generation Support**: M1, M2, M3, M4 with Pro/Max/Ultra variants
- **Sensor Definitions**: Structured mapping of sensors to platforms with metadata

### New Platform-Aware API
- `Smc::connect()` - Auto-detects platform
- `Smc::connect_with_platform(Platform)` - Manual platform selection
- `smc.platform()` - Get detected platform
- `smc.platform_cpu_core_temps()` - Platform-specific CPU core temperatures
- `smc.platform_gpu_temps()` - Platform-specific GPU temperatures
- `smc.has_sensor(key)` - Check sensor availability
- `smc.read_sensor(key)` - Read any sensor by key name

### Migration Notes
- Existing API remains unchanged and fully compatible
- New platform-aware methods provide enhanced functionality for modern Apple hardware
- Sensor keys now automatically adapt to detected platform