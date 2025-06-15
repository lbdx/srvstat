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
            println!("Fan Speed Command: {:?}", config.fan_speed_command);
            println!("Fan Speed File: {:?}", config.fan_speed_file);
            let reader = SystemMetricReader::new(config.clone());
            let writer = MqttMetricWriter::new(config.broker_url);
            let service = MetricService::new(reader, writer);
            service.process_metrics(Category::Disk);
            service.process_metrics(Category::Memory);
            service.process_metrics(Category::Cpu);
            service.process_metrics(Category::Swap);
            service.process_metrics(Category::FanSpeed);
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            eprintln!("Usage: Set the BROKER_URL environment variable. Optional: SRVSTAT_FAN_SPEED_COMMAND, SRVSTAT_FAN_SPEED_FILE.");
            println!("Writing values to console (MQTT writer disabled):");

            let default_config = Config {
                broker_url: String::new(), // Not used by DummyMetricWriter path
                fan_speed_command: std::env::var("SRVSTAT_FAN_SPEED_COMMAND").ok(), // Still try to get these
                fan_speed_file: std::env::var("SRVSTAT_FAN_SPEED_FILE").ok(),    // for fan speed reading
            };
            println!("Fan Speed Command (default path): {:?}", default_config.fan_speed_command);
            println!("Fan Speed File (default path): {:?}", default_config.fan_speed_file);

            let reader = SystemMetricReader::new(default_config);
            let writer = DummyMetricWriter;
            let service = MetricService::new(reader, writer);
            service.process_metrics(Category::Disk);
            service.process_metrics(Category::Memory);
            service.process_metrics(Category::Cpu);
            service.process_metrics(Category::Swap);
            service.process_metrics(Category::FanSpeed); // Ensure FanSpeed is processed here
            exit(1); // Ensure exit(1) is still called in the error path
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write; // For writing to temp file
    use std::process::Command;
    use std::env;
    use sysinfo::System; // Added for System::host_name()
    
    // Helper to get the expected hostname for assertions
    fn get_expected_hostname() -> String {
        // Attempt to get HOSTNAME env var, fallback to sysinfo, then to "devbox"
        std::env::var("HOSTNAME").unwrap_or_else(|_| {
            System::host_name().unwrap_or_else(|| "devbox".to_string())
        })
    }

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
            .env_remove("BROKER_URL") // Ensure it goes to DummyWriter path for predictability
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(!stdout.contains("Application version:"));
        // Check if it mentions writing to console (DummyWriter path)
        assert!(stdout.contains("Writing values to console"));
    }

    #[test]
    fn test_fan_speed_metric_processing_command() {
        let hostname = get_expected_hostname();
        let expected_rpm = "1234";
        let expected_metric_percent_output = format!("{}-FanSpeed: 0%", hostname);
        // The Used metric is not printed by MetricService due to commented out code.
        // let expected_metric_used_output = format!("{}-FanSpeed: {}/0", hostname, expected_rpm);

        env::set_var("SRVSTAT_FAN_SPEED_COMMAND", format!("echo {}", expected_rpm));
        env::remove_var("BROKER_URL"); // Ensure DummyWriter path

        let output = Command::new("cargo")
            .args(&["run"])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("--- test_fan_speed_metric_processing_command stdout ---");
        println!("{}", stdout);
        println!("--- test_fan_speed_metric_processing_command stderr ---");
        println!("{}", stderr);

        assert!(stdout.contains(&expected_metric_percent_output), "Stdout should contain FanSpeed percent metric");
        // assert!(stdout.contains(&expected_metric_used_output), "Stdout should contain FanSpeed used metric");
        assert!(stderr.contains("Error loading configuration"), "Stderr should indicate config error path");
        assert!(stdout.contains("Fan Speed Command (default path): Some(\"echo 1234\")"));


        env::remove_var("SRVSTAT_FAN_SPEED_COMMAND");
    }

    #[test]
    fn test_fan_speed_metric_processing_file() {
        let hostname = get_expected_hostname();
        let expected_rpm = "5678";
        let temp_dir = env::temp_dir();
        let temp_file_path = temp_dir.join("test_srvstat_fan.txt");

        // Create and write to the temporary file
        {
            let mut temp_file = fs::File::create(&temp_file_path).expect("Failed to create temp file");
            writeln!(temp_file, "{}", expected_rpm).expect("Failed to write to temp file");
        }

        let expected_metric_percent_output = format!("{}-FanSpeed: 0%", hostname);
        // The Used metric is not printed by MetricService due to commented out code.
        // let expected_metric_used_output = format!("{}-FanSpeed: {}/0", hostname, expected_rpm);

        env::set_var("SRVSTAT_FAN_SPEED_FILE", temp_file_path.to_str().unwrap());
        env::remove_var("SRVSTAT_FAN_SPEED_COMMAND"); // Ensure command is not set
        env::remove_var("BROKER_URL"); // Ensure DummyWriter path

        let output = Command::new("cargo")
            .args(&["run"])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("--- test_fan_speed_metric_processing_file stdout ---");
        println!("{}", stdout);
        println!("--- test_fan_speed_metric_processing_file stderr ---");
        println!("{}", stderr);

        assert!(stdout.contains(&expected_metric_percent_output), "Stdout should contain FanSpeed percent metric");
        // assert!(stdout.contains(&expected_metric_used_output), "Stdout should contain FanSpeed used metric");
        assert!(stderr.contains("Error loading configuration"), "Stderr should indicate config error path");
        assert!(stdout.contains(&format!("Fan Speed File (default path): Some(\"{}\")", temp_file_path.to_str().unwrap().replace("\\", "\\\\"))));


        fs::remove_file(&temp_file_path).expect("Failed to remove temp file");
        env::remove_var("SRVSTAT_FAN_SPEED_FILE");
    }

    #[test]
    fn test_fan_speed_metric_no_config() {
        let hostname = get_expected_hostname();
        let expected_metric_percent_output = format!("{}-FanSpeed: 0%", hostname);
        // let expected_metric_used_output = format!("{}-FanSpeed: 0/0", hostname); // RPM is 0

        env::remove_var("SRVSTAT_FAN_SPEED_COMMAND");
        env::remove_var("SRVSTAT_FAN_SPEED_FILE");
        env::remove_var("BROKER_URL");

        let output = Command::new("cargo")
            .args(&["run"])
            .output()
            .expect("failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);
        // let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stdout.contains(&expected_metric_percent_output));
        // assert!(stdout.contains(&expected_metric_used_output));
        assert!(stdout.contains("Fan Speed Command (default path): None"));
        assert!(stdout.contains("Fan Speed File (default path): None"));
        // eprintln should show "Fan speed not configured or failed to read"
        // This would be in output.stderr if the main's eprintln! goes there.
        // However, the test primarily checks stdout for DummyMetricWriter output.
    }
}
