// Platform-aware SMC sensor access example
use macsmc::*;

fn main() -> Result<()> {
    // Connect with automatic platform detection
    let mut smc = Smc::connect()?;
    
    println!("Detected platform: {:?}", smc.platform());
    
    // Get platform-specific CPU core temperatures
    match smc.platform_cpu_core_temps() {
        Ok(temps) => {
            println!("\nPlatform-specific CPU Core Temperatures:");
            for (key, temp_result) in temps {
                match temp_result {
                    Ok(temp) => println!("  {}: {:.1}°C", key, *temp),
                    Err(e) => println!("  {}: Error - {}", key, e),
                }
            }
        }
        Err(e) => println!("Error getting CPU temps: {}", e),
    }
    
    // Get platform-specific GPU temperatures
    match smc.platform_gpu_temps() {
        Ok(temps) => {
            println!("\nPlatform-specific GPU Temperatures:");
            for (key, temp_result) in temps {
                match temp_result {
                    Ok(temp) => println!("  {}: {:.1}°C", key, *temp),
                    Err(e) => println!("  {}: Error - {}", key, e),
                }
            }
        }
        Err(e) => println!("Error getting GPU temps: {}", e),
    }
    
    // Check for specific sensors
    let sensor_keys = ["Tp01", "Te05", "Tg0G", "TC0P", "TCGC"];
    println!("\nSensor availability:");
    for key in &sensor_keys {
        if smc.has_sensor(key) {
            if let Some(sensor_info) = smc.get_sensor_info(key) {
                println!("  ✓ {}: {} ({:?})", key, sensor_info.name, sensor_info.group);
            } else {
                println!("  ✓ {}: Available", key);
            }
        } else {
            println!("  ✗ {}: Not available", key);
        }
    }
    
    let power = smc.platform()
    println!("\nCPU Power: {:.2} W", power.core.0);
    Ok(())
}