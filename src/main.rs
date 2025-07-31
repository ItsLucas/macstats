//! macstats - Read system stats from macOS SMC with InfluxDB support
//!
//! Supports both console output and InfluxDB logging with configurable credentials.

mod config;
mod influx;

use clap::{Parser, Subcommand};
use config::{Config, MetricsConfig};
use influx::{InfluxClient, Metric};
use macsmc::{keys::*, SmcClient, SmcError};
use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{self, Display},
    time::{SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Smc(SmcError),
    Influx(influx::InfluxError),
    Config(Box<dyn StdError>),
    UnknownCommand(String),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Smc(e) => Some(e),
            Error::Influx(e) => Some(e),
            Error::Config(e) => Some(e.as_ref()),
            Error::UnknownCommand(_) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Smc(e) => write!(f, "SMC Error: {}", e),
            Error::Influx(e) => write!(f, "InfluxDB Error: {}", e),
            Error::Config(e) => write!(f, "Config Error: {}", e),
            Error::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
        }
    }
}

impl From<SmcError> for Error {
    fn from(e: SmcError) -> Self {
        Error::Smc(e)
    }
}

impl From<influx::InfluxError> for Error {
    fn from(e: influx::InfluxError) -> Self {
        Error::Influx(e)
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(e: Box<dyn StdError>) -> Self {
        Error::Config(e)
    }
}

#[derive(Parser)]
#[command(name = "macstats")]
#[command(about = "Read macOS SMC statistics with optional InfluxDB logging")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display CPU information
    Cpu,
    /// Display GPU information
    Gpu,
    /// Display system information
    System,
    /// Display power information
    Power,
    /// Display all information
    All,
    /// Send metrics to InfluxDB
    Influx {
        /// InfluxDB URL
        #[arg(long, default_value = "http://localhost:8086")]
        url: String,
        /// Database name (v1) or bucket name (v2)
        #[arg(long, default_value = "macstats")]
        database: String,
        /// Username (for v1 auth)
        #[arg(long)]
        username: Option<String>,
        /// Password (for v1 auth)
        #[arg(long)]
        password: Option<String>,
        /// Organization (for v2)
        #[arg(long)]
        org: Option<String>,
        /// Token (for v2)
        #[arg(long)]
        token: Option<String>,
        /// Bucket (for v2, defaults to database)
        #[arg(long)]
        bucket: Option<String>,
        /// Measurement prefix
        #[arg(long, default_value = "mac")]
        prefix: String,
        /// Additional tags (format: key=value,key2=value2)
        #[arg(long)]
        tags: Option<String>,
        /// Test connection only
        #[arg(long)]
        test: bool,
    },
    /// Configuration management
    Config {
        /// Show current configuration
        #[arg(long)]
        show: bool,
        /// Create example configuration file
        #[arg(long)]
        example: bool,
        /// Configuration file path
        #[arg(long)]
        path: bool,
    },
    /// Run continuous monitoring (requires config file)
    Monitor {
        /// Collection interval in seconds
        #[arg(short, long)]
        interval: Option<u64>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Cpu) => {
            let mut client = macsmc::connect()?;
            print_cpu_info(&mut client)?;
        }
        Some(Commands::Gpu) => {
            let mut client = macsmc::connect()?;
            print_gpu_info(&mut client)?;
        }
        Some(Commands::System) => {
            let mut client = macsmc::connect()?;
            print_system_info(&mut client)?;
        }
        Some(Commands::Power) => {
            let mut client = macsmc::connect()?;
            print_power_info(&mut client)?;
        }
        Some(Commands::All) => {
            let mut client = macsmc::connect()?;
            print_all_info(&mut client)?;
        }
        Some(Commands::Influx {
            url,
            database,
            username,
            password,
            org,
            token,
            bucket,
            prefix,
            tags,
            test,
        }) => {
            let mut influx_config = influx::InfluxConfig {
                url,
                database: database.clone(),
                username,
                password,
                org,
                token,
                bucket,
                measurement_prefix: Some(prefix),
                tags: parse_tags(tags)?,
            };

            let client = InfluxClient::new(influx_config)?;

            if test {
                println!("Testing InfluxDB connection...");
                client.test_connection().await?;
                println!("✓ Connection successful!");
            } else {
                send_to_influx(client).await?;
                println!("✓ Metrics sent to InfluxDB");
            }
        }
        Some(Commands::Config { show, example, path }) => {
            if show {
                match Config::load() {
                    Ok(config) => {
                        let toml = toml::to_string_pretty(&config).unwrap();
                        println!("{}", toml);
                    }
                    Err(e) => println!("Error loading config: {}", e),
                }
            } else if example {
                let example_config = Config::create_example();
                let config_path = Config::config_path()?;
                let toml = toml::to_string_pretty(&example_config).unwrap();
                println!("Example configuration for {}:\n", config_path.display());
                println!("{}", toml);
            } else if path {
                let config_path = Config::config_path()?;
                println!("{}", config_path.display());
            } else {
                println!("Use --show, --example, or --path");
            }
        }
        Some(Commands::Monitor { interval }) => {
            let config = Config::load()?;
            println!("Loaded configuration: {:?}", config);
            let interval = interval.unwrap_or(config.interval.unwrap_or(30));
            
            println!("Starting monitoring every {} seconds...", interval);
            println!("Press Ctrl+C to stop");

            let influx_client = InfluxClient::new(config.influx)?;
            
            loop {
                if let Err(e) = send_to_influx_with_config(&influx_client, &config.metrics).await {
                    eprintln!("Error sending metrics: {}", e);
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
            }
        }
        None => {
            // Default behavior - show CPU and power
            let mut client = macsmc::connect()?;
            print_cpu_info(&mut client)?;
            println!();
            print_power_info(&mut client)?;
        }
    }

    Ok(())
}

fn parse_tags(tags: Option<String>) -> Result<Option<HashMap<String, String>>> {
    if let Some(tags_str) = tags {
        let mut tag_map = HashMap::new();
        for pair in tags_str.split(',') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() == 2 {
                tag_map.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                return Err(Error::Config(format!("Invalid tag format: {}", pair).into()));
            }
        }
        Ok(Some(tag_map))
    } else {
        Ok(None)
    }
}

async fn send_to_influx(client: InfluxClient) -> Result<()> {
    let config = MetricsConfig::default();
    send_to_influx_with_config(&client, &config).await
}

async fn send_to_influx_with_config(client: &InfluxClient, config: &MetricsConfig) -> Result<()> {
    let mut smc_client = macsmc::connect()?;
    let mut metrics = Vec::new();

    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // CPU temperatures
    if config.cpu_temp {
        // M2 CPU cores
        for key in m2_cpu_temperature_keys() {
            if let Ok(data) = smc_client.read_key(key.key) {
                if let Ok(temp) = data.as_temperature() {
                    let metric = Metric::new("cpu_temperature", "value", *temp as f64)
                        .tag("host", &hostname)
                        .tag("core", &extract_core_name(key.name))
                        .tag("type", &extract_core_type(key.name));
                    metrics.push(metric);
                }
            }
        }

        // Universal CPU temperatures (can add more if needed)
        // for key in universal_cpu_temperature_keys() { ... }
    }

    // GPU temperatures
    if config.gpu_temp {
        for key in m2_gpu_temperature_keys() {
            if let Ok(data) = smc_client.read_key(key.key) {
                if let Ok(temp) = data.as_temperature() {
                    let metric = Metric::new("gpu_temperature", "value", *temp as f64)
                        .tag("host", &hostname)
                        .tag("gpu", &extract_gpu_number(key.name));
                    metrics.push(metric);
                }
            }
        }
    }

    // System temperatures
    if config.system_temp {
        for key in system_temperature_keys() {
            if let Ok(data) = smc_client.read_key(key.key) {
                if let Ok(temp) = data.as_temperature() {
                    let metric = Metric::new("system_temperature", "value", *temp as f64)
                        .tag("host", &hostname)
                        .tag("sensor", &key.name.to_lowercase().replace(' ', "_"));
                    metrics.push(metric);
                }
            }
        }
    }

    // Power metrics
    if config.power {
        for key in power_keys() {
            if let Ok(data) = smc_client.read_key(key.key) {
                if let Ok(power) = data.as_power() {
                    let metric = Metric::new("power", "value", *power as f64)
                        .tag("host", &hostname)
                        .tag("component", &key.name.to_lowercase().replace(' ', "_"));
                    metrics.push(metric);
                }
            }
        }

        // Voltage
        if let Ok(data) = smc_client.read_key("VD0R") {
            if let Ok(voltage) = data.as_voltage() {
                let metric = Metric::new("voltage", "value", *voltage as f64)
                    .tag("host", &hostname)
                    .tag("rail", "dc_in");
                metrics.push(metric);
            }
        }
    }

    // Fan speeds
    if config.fans {
        if let Ok(data) = smc_client.read_key("F0Ac") {
            if let Ok(rpm) = data.as_rpm() {
                let metric = Metric::new("fan_speed", "value", *rpm as f64)
                    .tag("host", &hostname)
                    .tag("fan", "0");
                metrics.push(metric);
            }
        }
    }

    if !metrics.is_empty() {
        client.write_metrics(metrics).await?;
    }

    Ok(())
}

fn extract_core_name(name: &str) -> String {
    if name.contains("Efficiency") {
        "efficiency".to_string()
    } else if let Some(num) = name.chars().last() {
        if num.is_ascii_digit() {
            format!("performance_{}", num)
        } else {
            "performance".to_string()
        }
    } else {
        "unknown".to_string()
    }
}

fn extract_core_type(name: &str) -> String {
    if name.contains("Efficiency") {
        "efficiency".to_string()
    } else if name.contains("Performance") {
        "performance".to_string()
    } else {
        "unknown".to_string()
    }
}

fn extract_gpu_number(name: &str) -> String {
    name.chars()
        .last()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_string())
        .unwrap_or_else(|| "0".to_string())
}

fn print_cpu_info(client: &mut SmcClient) -> Result<()> {
    println!("=== CPU Information ===");
    println!();
    
    for key in m2_cpu_temperature_keys() {
        if let Ok(data) = client.read_key(key.key) {
            if let Ok(temp) = data.as_temperature() {
                println!("{:>24}: {}", key.name, temp);
            }
        }
    }
    Ok(())
}

fn print_gpu_info(client: &mut SmcClient) -> Result<()> {
    println!("=== GPU Information ===");
    println!();
    
    for key in m2_gpu_temperature_keys() {
        if let Ok(data) = client.read_key(key.key) {
            if let Ok(temp) = data.as_temperature() {
                println!("{:>24}: {}", key.name, temp);
            }
        }
    }
    Ok(())
}

fn print_system_info(client: &mut SmcClient) -> Result<()> {
    println!("=== System Information ===");
    
    for key in system_temperature_keys() {
        if let Ok(data) = client.read_key(key.key) {
            if let Ok(temp) = data.as_temperature() {
                println!("{:>24}: {}", key.name, temp);
            }
        }
    }
    Ok(())
}

fn print_power_info(client: &mut SmcClient) -> Result<()> {
    println!("=== Power Information ===");
    
    for key in power_keys() {
        if let Ok(data) = client.read_key(key.key) {
            if let Ok(power) = data.as_power() {
                println!("{:>24}: {}", key.name, power);
            }
        }
    }

    // Voltage
    if let Ok(data) = client.read_key("VD0R") {
        if let Ok(voltage) = data.as_voltage() {
            println!("{:>24}: {}", "DC In", voltage);
        }
    }

    Ok(())
}

fn print_fan_info(client: &mut SmcClient) -> Result<()> {
    println!("=== Fan Information ===");
    
    if let Ok(data) = client.read_key("F0Ac") {
        if let Ok(rpm) = data.as_rpm() {
            println!("{:>24}: {}", "Fan Speed", rpm);
        }
    }
    Ok(())
}

fn print_all_info(client: &mut SmcClient) -> Result<()> {
    print_cpu_info(client)?;
    println!();
    print_gpu_info(client)?;
    println!();
    print_system_info(client)?;
    println!();
    print_power_info(client)?;
    println!();
    print_fan_info(client)?;
    Ok(())
}