use crate::config::Config;
use crate::domain::metrics::models::{Category, Metric, Percentage};
use crate::domain::ports::MetricReader;
use std::process::Command;
use sysinfo::{Disks, System}; // Removed unused sysinfo::Components

pub struct DummyMetricReader;
impl MetricReader for DummyMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        Metric::Percent(
            "tux".to_string(),
            category.clone(),
            Percentage::new(25).unwrap(),
        )
    }
    fn get_used(&self, category: &Category) -> Metric {
        Metric::Used("tux".to_string(), category.clone(), 25, 100)
    }
}

pub struct SystemMetricReader {
    config: Config,
}

impl SystemMetricReader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl MetricReader for SystemMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        let mut sys = System::new_all();
        let host = System::host_name().unwrap_or_else(|| "unknown_host".to_string());
        match category {
            Category::Cpu => {
                sys.refresh_cpu_usage();
                let cpu_usage = sys.global_cpu_usage() as u8;
                Metric::Percent(host, category.clone(), Percentage::new(cpu_usage).unwrap())
            }
            Category::FanSpeed => {
                let used_metric_obj = self.get_used(category);
                match used_metric_obj {
                    Metric::Used(_host_from_used, _cat_from_used, rpm_value, total_metric) => {
                        // total_metric for FanSpeed from get_used is currently 0.
                        // Percentage is (rpm_value / total_metric) * 100.
                        // If total_metric is 0, this results in 0 percent.
                        let percentage_val = if total_metric == 0 {
                            0u8
                        } else {
                            (rpm_value as f64 / total_metric as f64 * 100.0).round() as u8
                        };
                        Metric::Percent(host, category.clone(), Percentage::new(percentage_val).unwrap_or(Percentage(0)))
                    }
                    _ => Metric::Percent(host, category.clone(), Percentage::new(0).unwrap()), // Should not happen
                }
            }
            _ => { // Handles Disk, Memory, Swap
                let used = self.get_used(category);
                match used {
                    Metric::Used(_host_from_used, _cat_from_used, used_metric, total_metric) => {
                        let usage_percent = if total_metric == 0 {
                            0.0
                        } else {
                            used_metric as f64 / total_metric as f64 * 100.0
                        };
                        let usage_percent_u8 = usage_percent.round() as u8;
                        Metric::Percent(
                            host,
                            category.clone(),
                            Percentage::new(usage_percent_u8).unwrap_or_else(|err| {
                                eprintln!(
                                    "Invalid percentage calculated: {} for {}/{} (Error: {})",
                                    usage_percent_u8, used_metric, total_metric, err
                                );
                                Percentage(0)
                            }),
                        )
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn get_used(&self, category: &Category) -> Metric {
        let mut sys = System::new_all(); // sys is only used for Disk, Memory, Swap
        let host = System::host_name().unwrap_or_else(|| "unknown_host".to_string());
        match category {
            Category::Disk => {
                sys.refresh_all();
                let disks = Disks::new_with_refreshed_list();
                let disk = disks.first().expect("No disk found for sysinfo reader"); // Or handle error
                let total_space = disk.total_space();
                let available_space = disk.available_space();
                let used_space = total_space - available_space;
                Metric::Used(host, category.clone(), used_space, total_space)
            }
            Category::Memory => {
                sys.refresh_memory();
                Metric::Used(
                    host,
                    category.clone(),
                    sys.used_memory(),
                    sys.total_memory(),
                )
            }
            Category::Cpu => {
                eprintln!("Error: no used metric for cpu");
                Metric::Used(host, category.clone(), 0, 0)
            }
            Category::Swap => {
                sys.refresh_memory();
                let used_swap = sys.used_swap();
                let total_swap = sys.total_swap();
                Metric::Used(
                    host,
                    category.clone(),
                    used_swap,
                    total_swap,
                )
            }
            Category::FanSpeed => {
                if let Some(command_str) = &self.config.fan_speed_command {
                    let parts: Vec<&str> = command_str.split_whitespace().collect();
                    if !parts.is_empty() {
                        let cmd_name = parts[0];
                        let args = &parts[1..];
                        match Command::new(cmd_name).args(args).output() {
                            Ok(output) => {
                                if output.status.success() {
                                    match String::from_utf8(output.stdout) {
                                        Ok(stdout_str) => {
                                            match stdout_str.trim().parse::<u64>() {
                                                Ok(rpm_value) => {
                                                    return Metric::Used(host, category.clone(), rpm_value, 0);
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to parse fan speed command output '{}' to u64: {}", stdout_str.trim(), e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Fan speed command output was not valid UTF-8: {}", e);
                                        }
                                    }
                                } else {
                                    eprintln!(
                                        "Fan speed command '{}' failed with status: {}. Stderr: {}",
                                        command_str,
                                        output.status,
                                        String::from_utf8_lossy(&output.stderr)
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to execute fan speed command '{}': {}", command_str, e);
                            }
                        }
                    } else {
                        eprintln!("Fan speed command string was empty after split: '{}'", command_str);
                    }
                }

                if let Some(file_path) = &self.config.fan_speed_file {
                    match std::fs::read_to_string(file_path) {
                        Ok(content) => {
                            match content.trim().parse::<u64>() {
                                Ok(rpm_value) => {
                                    return Metric::Used(host, category.clone(), rpm_value, 0);
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse fan speed file content '{}' from '{}' to u64: {}", content.trim(), file_path, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read fan speed file '{}': {}", file_path, e);
                        }
                    }
                }

                eprintln!("Fan speed not configured via command/file or failed to read. Returning 0 RPM.");
                Metric::Used(host, category.clone(), 0, 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn create_dummy_config(cmd: Option<String>, file: Option<String>) -> Config {
        Config {
            broker_url: "dummy_url".to_string(), // Not used by these specific tests
            fan_speed_command: cmd,
            fan_speed_file: file,
        }
    }

    #[test]
    fn test_fan_speed_from_command_ok() {
        let config = create_dummy_config(Some("echo 750".to_string()), None);
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed);
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 750);
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_command_whitespace() {
        // Test that `trim()` handles whitespace around the number from command output.
        // `echo 123` will output "123\n". stdout.trim() will be "123".
        // This test relies on `trim()` more than on complex shell output.
        let config = create_dummy_config(Some("echo 123".to_string()), None); // Was causing issues with complex echo/printf
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed);
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 123);
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_command_invalid_output() {
        let config = create_dummy_config(Some("echo not_a_number".to_string()), None);
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed); // eprint_ln expected
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 0); // Should fallback to 0
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_command_non_existent() {
        let config = create_dummy_config(Some("non_existent_command_12345".to_string()), None);
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed); // eprint_ln expected
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 0); // Should fallback to 0
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_file_ok() {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("test_fan_ok.txt");
        std::fs::write(&temp_file_path, "800").expect("Failed to write temp file");

        let config = create_dummy_config(None, Some(temp_file_path.to_str().unwrap().to_string()));
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed);

        std::fs::remove_file(&temp_file_path).expect("Failed to remove temp file");

        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 800);
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_file_whitespace() {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("test_fan_whitespace.txt");
        std::fs::write(&temp_file_path, "  234 \n ").expect("Failed to write temp file");

        let config = create_dummy_config(None, Some(temp_file_path.to_str().unwrap().to_string()));
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed);

        std::fs::remove_file(&temp_file_path).expect("Failed to remove temp file");

        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 234);
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_file_invalid_content() {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("test_fan_invalid.txt");
        std::fs::write(&temp_file_path, "not_rpm").expect("Failed to write temp file");

        let config = create_dummy_config(None, Some(temp_file_path.to_str().unwrap().to_string()));
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed); // eprintln expected

        std::fs::remove_file(&temp_file_path).expect("Failed to remove temp file");

        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 0); // Fallback
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_from_file_non_existent() {
        let config = create_dummy_config(None, Some("/tmp/non_existent_fan_file_12345.txt".to_string()));
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed); // eprintln expected
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 0); // Fallback
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_no_config() {
        let config = create_dummy_config(None, None);
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed); // eprintln expected
        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 0);
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_command_takes_precedence() {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("test_fan_precedence.txt");
        std::fs::write(&temp_file_path, "999").expect("Failed to write temp file");

        // Command should be used, file should be ignored
        let config = create_dummy_config(Some("echo 666".to_string()), Some(temp_file_path.to_str().unwrap().to_string()));
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_used(&Category::FanSpeed);

        std::fs::remove_file(&temp_file_path).expect("Failed to remove temp file");

        if let Metric::Used(_, Category::FanSpeed, val, total) = metric {
            assert_eq!(val, 666); // Value from command
            assert_eq!(total, 0);
        } else {
            panic!("Expected Metric::Used for FanSpeed, got {:?}", metric);
        }
    }

    #[test]
    fn test_fan_speed_percent_calculation() {
        // get_used for FanSpeed returns RPM as 'used' and 0 as 'total'.
        // So, get_percent should result in 0%.
        let config = create_dummy_config(Some("echo 750".to_string()), None);
        let reader = SystemMetricReader::new(config);
        let metric = reader.get_percent(&Category::FanSpeed);
        if let Metric::Percent(_, Category::FanSpeed, percentage) = metric {
            assert_eq!(percentage.0, 0);
        } else {
            panic!("Expected Metric::Percent for FanSpeed, got {:?}", metric);
        }

        // Test with no config, should also be 0%
        let config_no = create_dummy_config(None, None);
        let reader_no = SystemMetricReader::new(config_no);
        let metric_no = reader_no.get_percent(&Category::FanSpeed);
        if let Metric::Percent(_, Category::FanSpeed, percentage) = metric_no {
            assert_eq!(percentage.0, 0);
        } else {
            panic!("Expected Metric::Percent for FanSpeed, got {:?}", metric_no);
        }
    }
}
