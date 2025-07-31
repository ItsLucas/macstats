# macstats InfluxDB Integration

Complete guide for sending macOS SMC metrics to InfluxDB.

## Quick Start

### 1. Basic Usage (Command Line)

Send all metrics to InfluxDB with command-line credentials:

```bash
# InfluxDB v1 with username/password
./macstats influx --url http://localhost:8086 --username admin --password secret

# InfluxDB v2 with token  
./macstats influx --url http://localhost:8086 --org myorg --token your_token_here

# Test connection first
./macstats influx --test --url http://localhost:8086 --token your_token_here
```

### 2. Configuration File Setup

Create a persistent configuration for continuous monitoring:

```bash
# Show example config
./macstats config --example

# Show config file location
./macstats config --path
# Output: /Users/your_name/Library/Application Support/macstats/config.toml
```

Create/edit the config file:

```toml
hostname = "my-macbook-m2"
interval = 30  # seconds

[influx]
url = "http://localhost:8086"
database = "macstats"          # For v1, or bucket name for v2
measurement_prefix = "mac"     # Prefix for measurements

# For InfluxDB v1 authentication
username = "admin"
password = "your_password"

# For InfluxDB v2 authentication (comment out v1 auth above)
# org = "your_organization" 
# token = "your_influxdb_token"
# bucket = "macstats"

# Global tags added to all metrics
[influx.tags]
host = "my-macbook-m2"
location = "office"
environment = "production"

# Control which metrics to collect
[metrics]
cpu_temp = true     # M2 CPU core temperatures
gpu_temp = true     # M2 GPU temperatures
system_temp = true  # NAND, Airport, etc.
power = true        # Power consumption, voltage
fans = true         # Fan speeds
```

### 3. Continuous Monitoring

Start continuous monitoring using the config file:

```bash
# Use default interval from config (30s)
./macstats monitor

# Override interval
./macstats monitor --interval 60

# Monitor runs continuously until Ctrl+C
Starting monitoring every 30 seconds...
Press Ctrl+C to stop
```

## InfluxDB Data Structure

### Measurements Created:

1. **`mac_cpu_temperature`** - CPU core temperatures
   - Tags: `host`, `core`, `type`
   - Field: `value` (°C)

2. **`mac_gpu_temperature`** - GPU temperatures
   - Tags: `host`, `gpu`
   - Field: `value` (°C)

3. **`mac_system_temperature`** - System sensors
   - Tags: `host`, `sensor`
   - Field: `value` (°C)

4. **`mac_power`** - Power consumption
   - Tags: `host`, `component`
   - Field: `value` (W)

5. **`mac_voltage`** - Voltage readings
   - Tags: `host`, `rail`
   - Field: `value` (V)

6. **`mac_fan_speed`** - Fan speeds
   - Tags: `host`, `fan`
   - Field: `value` (RPM)

### Example Data Points:

```
mac_cpu_temperature,host=my-macbook-m2,core=performance_1,type=performance value=42.3 1640995200000000000
mac_power,host=my-macbook-m2,component=system_total_power value=6.05 1640995200000000000
mac_voltage,host=my-macbook-m2,rail=dc_in value=12.224 1640995200000000000
```

## Advanced Configuration

### Custom Tags

Add custom tags via command line:

```bash
./macstats influx --tags "rack=R1,datacenter=US-West" --token your_token
```

Or in config file:

```toml
[influx.tags]
rack = "R1"
datacenter = "US-West"
team = "platform"
```

### Different Measurement Prefix

```bash
./macstats influx --prefix "apple_silicon" --token your_token
```

Results in measurements like: `apple_silicon_cpu_temperature`

### Selective Metrics

In config file, disable unwanted metrics:

```toml
[metrics]
cpu_temp = true
gpu_temp = false    # Skip GPU metrics
system_temp = true
power = true
fans = false        # Skip fan metrics
```

## InfluxDB Setup Examples

### InfluxDB v1 Setup

```bash
# Install InfluxDB v1
brew install influxdb@1

# Start service
brew services start influxdb@1

# Create database
influx -execute 'CREATE DATABASE macstats'

# Create user (optional)
influx -execute 'CREATE USER admin WITH PASSWORD "secret" WITH ALL PRIVILEGES'
```

Use with macstats:
```bash
./macstats influx --username admin --password secret
```

### InfluxDB v2 Setup

```bash
# Install InfluxDB v2
brew install influxdb

# Start service  
brew services start influxdb

# Setup via web UI at http://localhost:8086
# Or via CLI:
influx setup --username admin --password your_password --org myorg --bucket macstats
```

Get your token:
```bash
influx auth list
```

Use with macstats:
```bash
./macstats influx --org myorg --token your_token_here --bucket macstats
```

## Grafana Dashboard

Example Grafana queries for the collected data:

### CPU Temperature Panel:
```sql
SELECT mean("value") FROM "mac_cpu_temperature" 
WHERE ("host" = 'my-macbook-m2') AND $timeFilter 
GROUP BY time($__interval), "core" fill(null)
```

### Power Consumption Panel:
```sql
SELECT mean("value") FROM "mac_power" 
WHERE ("host" = 'my-macbook-m2') AND $timeFilter 
GROUP BY time($__interval), "component" fill(null)
```

### System Health Single Stat:
```sql
SELECT mean("value") FROM "mac_cpu_temperature" 
WHERE ("host" = 'my-macbook-m2') AND ("type" = 'performance') AND $timeFilter
```

## Troubleshooting

### Connection Issues

Test connectivity:
```bash
./macstats influx --test --url http://your-influxdb:8086 --token your_token
```

Common issues:
- **404 errors**: Check URL and InfluxDB version (v1 vs v2 endpoints)
- **401 errors**: Check credentials (username/password or token)
- **Network errors**: Check firewall, DNS, and connectivity

### Data Not Appearing

1. **Check metrics are enabled** in config:
   ```bash
   ./macstats config --show
   ```

2. **Verify SMC data** is readable:
   ```bash
   ./macstats all
   ```

3. **Check InfluxDB logs**:
   ```bash
   # v1
   tail -f /usr/local/var/log/influxdb.log
   
   # v2  
   influx server-config
   ```

### Performance

- **High CPU usage**: Increase monitoring interval
- **Too much data**: Disable unnecessary metrics in config
- **Network load**: Use local InfluxDB instance

## Integration Examples

### Systemd Service (Linux)

Create `/etc/systemd/system/macstats.service`:

```ini
[Unit]
Description=macstats monitoring
After=network.target

[Service]
Type=simple
User=monitor
ExecStart=/usr/local/bin/macstats monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Docker Compose

```yaml
version: '3.8'
services:
  influxdb:
    image: influxdb:2.7
    environment:
      - DOCKER_INFLUXDB_INIT_MODE=setup
      - DOCKER_INFLUXDB_INIT_USERNAME=admin
      - DOCKER_INFLUXDB_INIT_PASSWORD=password
      - DOCKER_INFLUXDB_INIT_ORG=myorg
      - DOCKER_INFLUXDB_INIT_BUCKET=macstats
    ports:
      - "8086:8086"
    volumes:
      - influxdb-data:/var/lib/influxdb2

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-data:/var/lib/grafana

volumes:
  influxdb-data:
  grafana-data:
```

This gives you a complete monitoring stack for your M2 Mac's SMC data!