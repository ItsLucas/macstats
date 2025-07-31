//! InfluxDB integration for macstats

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error as StdError, fmt, time::{SystemTime, UNIX_EPOCH}};

/// InfluxDB configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluxConfig {
    /// InfluxDB URL (e.g., http://localhost:8086)
    pub url: String,
    /// Database name
    pub database: String,
    /// Optional username
    pub username: Option<String>,
    /// Optional password
    pub password: Option<String>,
    /// Optional organization (for InfluxDB v2)
    pub org: Option<String>,
    /// Optional token (for InfluxDB v2)
    pub token: Option<String>,
    /// Optional bucket (for InfluxDB v2, defaults to database name)
    pub bucket: Option<String>,
    /// Measurement name prefix
    pub measurement_prefix: Option<String>,
    /// Additional tags to add to all metrics
    pub tags: Option<HashMap<String, String>>,
}

impl Default for InfluxConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:8086".to_string(),
            database: "macstats".to_string(),
            username: None,
            password: None,
            org: None,
            token: None,
            bucket: None,
            measurement_prefix: Some("mac".to_string()),
            tags: None,
        }
    }
}

/// InfluxDB client
pub struct InfluxClient {
    config: InfluxConfig,
    client: Client,
}

/// InfluxDB error
#[derive(Debug)]
pub enum InfluxError {
    /// HTTP request error
    Http(reqwest::Error),
    /// Configuration error
    Config(String),
    /// InfluxDB server error
    Server { status: u16, message: String },
}

impl fmt::Display for InfluxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfluxError::Http(e) => write!(f, "HTTP error: {}", e),
            InfluxError::Config(msg) => write!(f, "Configuration error: {}", msg),
            InfluxError::Server { status, message } => {
                write!(f, "InfluxDB server error {}: {}", status, message)
            }
        }
    }
}

impl StdError for InfluxError {}

impl From<reqwest::Error> for InfluxError {
    fn from(err: reqwest::Error) -> Self {
        InfluxError::Http(err)
    }
}

type Result<T> = std::result::Result<T, InfluxError>;

/// Metric data point
#[derive(Debug, Clone)]
pub struct Metric {
    /// Measurement name
    pub measurement: String,
    /// Field name and value
    pub field: String,
    /// Field value
    pub value: f64,
    /// Additional tags
    pub tags: HashMap<String, String>,
    /// Timestamp in nanoseconds (None = current time)
    pub timestamp: Option<u64>,
}

impl Metric {
    /// Create a new metric
    pub fn new(measurement: &str, field: &str, value: f64) -> Self {
        Self {
            measurement: measurement.to_string(),
            field: field.to_string(),
            value,
            tags: HashMap::new(),
            timestamp: None,
        }
    }

    /// Add a tag
    pub fn tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: u64) -> Self {
        self.timestamp = Some(ts);
        self
    }
}

impl InfluxClient {
    /// Create a new InfluxDB client
    pub fn new(config: InfluxConfig) -> Result<Self> {
        let client = Client::new();
        Ok(Self { config, client })
    }

    /// Write metrics to InfluxDB
    pub async fn write_metrics(&self, metrics: Vec<Metric>) -> Result<()> {
        if metrics.is_empty() {
            return Ok(());
        }

        let line_protocol = self.format_line_protocol(&metrics)?;
        
        // Determine API version and build request
        if self.config.token.is_some() {
            self.write_v2(&line_protocol).await
        } else {
            self.write_v1(&line_protocol).await
        }
    }

    /// Write using InfluxDB v1 API
    async fn write_v1(&self, data: &str) -> Result<()> {
        let mut url = format!("{}/write?db={}", self.config.url, self.config.database);
        
        let mut request = self.client.post(&url);

        // Add authentication if provided
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            request = request.basic_auth(username, Some(password));
        }

        let response = request
            .header("Content-Type", "application/octet-stream")
            .body(data.to_string())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Write using InfluxDB v2 API
    async fn write_v2(&self, data: &str) -> Result<()> {
        let org = self.config.org.as_ref().ok_or_else(|| {
            InfluxError::Config("Organization required for InfluxDB v2".to_string())
        })?;
        
        let bucket = self.config.bucket.as_ref().unwrap_or(&self.config.database);
        
        let token = self.config.token.as_ref().ok_or_else(|| {
            InfluxError::Config("Token required for InfluxDB v2".to_string())
        })?;

        let url = format!(
            "{}/api/v2/write?org={}&bucket={}",
            self.config.url, org, bucket
        );

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(data.to_string())
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response
    async fn handle_response(&self, response: reqwest::Response) -> Result<()> {
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(InfluxError::Server {
                status: status.as_u16(),
                message,
            })
        }
    }

    /// Format metrics as InfluxDB line protocol
    fn format_line_protocol(&self, metrics: &[Metric]) -> Result<String> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| InfluxError::Config(format!("Time error: {}", e)))?
            .as_nanos() as u64;

        let mut lines = Vec::new();

        for metric in metrics {
            let measurement = if let Some(prefix) = &self.config.measurement_prefix {
                format!("{}_{}", prefix, metric.measurement)
            } else {
                metric.measurement.clone()
            };

            // Build tags
            let mut all_tags = HashMap::new();
            
            // Add global tags
            if let Some(global_tags) = &self.config.tags {
                all_tags.extend(global_tags.clone());
            }
            
            // Add metric-specific tags
            all_tags.extend(metric.tags.clone());

            // Format measurement and tags
            let mut line = measurement;
            if !all_tags.is_empty() {
                let tag_string: Vec<String> = all_tags
                    .iter()
                    .map(|(k, v)| format!("{}={}", escape_tag_key(k), escape_tag_value(v)))
                    .collect();
                line.push(',');
                line.push_str(&tag_string.join(","));
            }

            // Add field
            line.push(' ');
            line.push_str(&format!("{}={}", 
                escape_field_key(&metric.field), 
                format_field_value(metric.value)
            ));

            // Add timestamp
            let timestamp = metric.timestamp.unwrap_or(current_time);
            line.push(' ');
            line.push_str(&timestamp.to_string());

            lines.push(line);
        }

        Ok(lines.join("\n"))
    }

    /// Test connection to InfluxDB
    pub async fn test_connection(&self) -> Result<()> {
        let url = if self.config.token.is_some() {
            format!("{}/health", self.config.url)
        } else {
            format!("{}/ping", self.config.url)
        };

        let mut request = self.client.get(&url);

        if let Some(token) = &self.config.token {
            request = request.header("Authorization", format!("Token {}", token));
        } else if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            request = request.basic_auth(username, Some(password));
        }

        let response = request.send().await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(InfluxError::Server {
                status: response.status().as_u16(),
                message: "Connection test failed".to_string(),
            })
        }
    }
}

// Helper functions for InfluxDB line protocol escaping
fn escape_tag_key(s: &str) -> String {
    s.replace(',', "\\,").replace(' ', "\\ ").replace('=', "\\=")
}

fn escape_tag_value(s: &str) -> String {
    s.replace(',', "\\,").replace(' ', "\\ ").replace('=', "\\=")
}

fn escape_field_key(s: &str) -> String {
    s.replace(',', "\\,").replace(' ', "\\ ").replace('=', "\\=")
}

fn format_field_value(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
        format!("{}i", value as i64)
    } else {
        value.to_string()
    }
}