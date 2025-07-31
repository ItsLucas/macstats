// Example demonstrating platform-aware SMC sensor access
use macsmc::*;

fn main() -> Result<()> {
    println!("=== Platform-Aware SMC Demo ===\n");
    
    // Connect with automatic platform detection
    let mut smc = Smc::connect()?;
    
    println!("Detected platform: {:?}", smc.platform());
    
    // Test different platform connections
    println!("\n=== Platform Comparison ===");
    for platform in [Platform::M1, Platform::M2, Platform::M3, Platform::M4] {
        let smc_test = Smc::connect_with_platform(platform)?;
        let cpu_keys: Vec<_> = match smc_test.platform_cpu_core_temps() {
            Ok(temps) => temps.into_iter().map(|(k, _)| k).collect(),
            Err(_) => vec![]
        };
        println!("{:?}: {} CPU core sensors available", platform, cpu_keys.len());
        if !cpu_keys.is_empty() {
            println!("  Keys: {:?}", cpu_keys);
        }
    }
    
    // Test sensor availability
    println!("\n=== Sensor Availability Check ===");
    let test_sensors = [
        ("TC0P", "CPU Proximity (Universal)"),
        ("Tp01", "M1/M2/M4 Performance Core 1"),
        ("Te05", "M3/M4 Efficiency Core 1"), 
        ("Tf04", "M3 Performance Core 1"),
        ("Tg0G", "M4 GPU 1"),
        ("TCGC", "Intel Graphics"),
    ];
    
    for (key, description) in &test_sensors {
        let available = smc.has_sensor(key);
        let status = if available { "✓" } else { "✗" };
        println!("{} {}: {}", status, key, description);
        
        if available {
            if let Some(sensor_info) = smc.get_sensor_info(key) {
                println!("    Group: {:?}, Type: {:?}", sensor_info.group, sensor_info.sensor_type);
            }
        }
    }
    
    // Try to read platform-specific CPU temperatures
    println!("\n=== Platform-Specific CPU Temperatures ===");
    match smc.platform_cpu_core_temps() {
        Ok(temps) => {
            for (key, temp_result) in temps {
                match temp_result {
                    Ok(temp) => println!("  {}: {:.1}°C", key, *temp),
                    Err(_) => println!("  {}: <not available>", key),
                }
            }
        }
        Err(e) => println!("Error reading CPU temperatures: {}", e),
    }
    
    // Try to read platform-specific GPU temperatures  
    println!("\n=== Platform-Specific GPU Temperatures ===");
    match smc.platform_gpu_temps() {
        Ok(temps) => {
            if temps.is_empty() {
                println!("  No platform-specific GPU sensors found");
            } else {
                for (key, temp_result) in temps {
                    match temp_result {
                        Ok(temp) => println!("  {}: {:.1}°C", key, *temp),
                        Err(_) => println!("  {}: <not available>", key),
                    }
                }
            }
        }
        Err(e) => println!("Error reading GPU temperatures: {}", e),
    }
    
    println!("\n=== Legacy API Still Works ===");
    // Show that the old API still works
    match smc.cpu_temperature() {
        Ok(cpu_temp) => {
            println!("CPU Proximity: {:.1}°C", *cpu_temp.proximity);
            println!("CPU Die: {:.1}°C", *cpu_temp.die);
        }
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\nDemo complete!");
    Ok(())
}