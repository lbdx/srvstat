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
//! 3. Set environment variables for configuration (such as server port):
//!     ```bash
//!     export SERVER_PORT=8080
//!

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
    let config = Config::from_env().expect("Error reading parameters.");
    if config.server_port.is_none() {
        println!("Config server_port={:?}", config.server_port);
        let reader = SystemMetricReader;
        let writer = MqttMetricWriter::new("tcp://192.168.1.70:1883");
        let service = MetricService::new(reader, writer);
        service.process_metrics(Category::Disk);
        service.process_metrics(Category::Memory);
        service.process_metrics(Category::Cpu);
    } else {
        let reader = SystemMetricReader;
        let writer = DummyMetricWriter;
        let service = MetricService::new(reader, writer); // Create an instance of DummyMetricService
        service.process_metrics(Category::Disk);
        service.process_metrics(Category::Memory);
        service.process_metrics(Category::Cpu);
    }
}
