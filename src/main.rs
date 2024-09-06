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

use std::process::exit;
use crate::domain::metrics::metric_service::MetricService;
use crate::domain::metrics::models::Category;
use crate::domain::ports::MetricProcessor;
use crate::outbound::metric_writer::MqttMetricWriter;
use config::Config;
use outbound::{metric_reader::SystemMetricReader, metric_writer::DummyMetricWriter};

pub mod config;
pub mod domain;
pub mod outbound;

fn main() {
    match Config::from_env() {
        Ok(config) => {
            println!("Config broker_url={:?}", config.broker_url);
            let reader = SystemMetricReader;
            let writer = MqttMetricWriter::new(config.broker_url);
            let service = MetricService::new(reader, writer);
            service.process_metrics(Category::Disk);
            service.process_metrics(Category::Memory);
            service.process_metrics(Category::Cpu);
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
            exit(1);
        }
    }
}
