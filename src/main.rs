//! # Metrics Publisher
//!
//! A lightweight Rust application that reads system metrics (such as Disk, CPU, and Memory usage) from the host and publishes them to an MQTT queue.
//!
//! ## Features
//!
//! - Reads metrics from the system.
//! - Publishes metrics to an MQTT queue.
//! - Supports disk, CPU, and memory metrics.
//!
//! ## Project Structure
//!
//! - `config`: Contains configuration settings for the application.
//! - `domain`: Defines core domain logic, such as metric categories and services.
//! - `outbound`: Handles outbound operations, including reading and writing metrics.
//!
//! ## Usage
//!
//! 1. Clone the repository:
//!     ```bash
//!     git clone <repository-url>
//!     ```
//!
//! 2. Build the project:
//!     ```bash
//!     cargo build --release
//!     ```
//!
//! 3. Set environment variables for configuration (such as BROKER_URL):
//!     ```bash
//!     export BROKER_URL=tcp://localhost:1883
//!

use crate::domain::metrics::metric_service::MetricService;
use crate::domain::metrics::models::Category;
use crate::domain::ports::MetricProcessor;
use crate::outbound::metric_writer::MqttMetricWriter;
use config::Config;
use outbound::{metric_reader::SystemMetricReader, metric_writer::DummyMetricWriter};
use std::process::exit;
use std::env;

pub mod config;
pub mod domain;
pub mod outbound;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        println!("Application version: {}", version);
        return;
    }

    match Config::from_env() {
        Ok(config) => {
            println!("Config broker_url={:?}", config.broker_url);
            let reader = SystemMetricReader;
            let writer = MqttMetricWriter::new(config.broker_url);
            let service = MetricService::new(reader, writer);
            service.process_metrics(Category::Disk);
            service.process_metrics(Category::Memory);
            service.process_metrics(Category::Cpu);
            service.process_metrics(Category::Swap);
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            eprintln!("Usage: Set the BROKER_URL environment variable.");
            println!("Writing values to console :");
            let reader = SystemMetricReader;
            let writer = DummyMetricWriter;
            let service = MetricService::new(reader, writer);
            service.process_metrics(Category::Disk);
            service.process_metrics(Category::Memory);
            service.process_metrics(Category::Cpu);
            service.process_metrics(Category::Swap);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;
    
    #[test]
    fn test_version_flag() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = env!("CARGO_PKG_VERSION");

        assert!(stdout.contains(&format!("Application version: {}", version)));
    }

    #[test]
    fn test_no_version_flag() {
        let output = Command::new("cargo")
            .args(&["run"])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(!stdout.contains("Application version:"));
    }
}
