use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents different metrics that can be tracked.
#[derive(Debug, PartialEq)]
pub enum Metric {
    /// A percentage-based metric (0-100) for a specific category.
    ///
    /// # Parameters
    /// * `name`: The name of the metric (e.g., "CPU Usage").
    /// * `category`: The category of the metric (e.g., "Disk").
    /// * `percentage`: The value as a percentage (0-100).
    Percent(String, Category, Percentage),
    /// A used/total value pair for a specific category, such as memory usage.
    ///
    /// # Parameters
    /// * `name`: The name of the metric (e.g., "Memory Usage").
    /// * `category`: The category of the metric (e.g., "Disk").
    /// * `used`: The amount of the resource being used (e.g., 1000 MB).
    /// * `total`: The total amount of the resource available (e.g., 4000 MB).
    Used(String, Category, u64, u64),
    /// A generic metric value for a specific component.
    Value {
        host: String,
        category: Category,
        component_label: String,
        value: f32,
        unit: String,
    },
}

// ComponentTemperature struct is removed.

/// Represents the different categories of resources that can be measured.
#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    Disk,
    Memory,
    Cpu,
    Swap,
    Temperature,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Metric::Percent(host, category, Percentage(value)) => {
                write!(f, "{}-{}: {}%", host, category, value)
            }
            Metric::Used(host, category, used, total) => {
                write!(f, "{}-{}: {}/{}", host, category, used, total)
            }
            Metric::Value {
                host,
                category,
                component_label,
                value,
                unit,
            } => write!(
                f,
                "{host} - {category} - {label}: {value}{unit}",
                host = host,
                category = category,
                label = component_label,
                value = value,
                unit = unit
            ),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Disk => write!(f, "Disk"),
            Category::Memory => write!(f, "Memory"),
            Category::Cpu => write!(f, "CPU"),
            Category::Swap => write!(f, "Swap"),
            Category::Temperature => write!(f, "temperature"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Percentage(pub u8);

#[derive(Clone, Debug, Error, PartialEq)]
#[error("Percent value must be between 0 and 100")]
pub struct InvalidPercentage;

impl Percentage {
    pub fn new(value: u8) -> Result<Self, InvalidPercentage> {
        if value > 100 {
            Err(InvalidPercentage)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    // Ensure ComponentTemperature is not imported if it was before
    use super::{Category, Metric, Percentage}; // Adjusted imports
    fn host() -> String {
        "test".to_string()
    }

    #[test]
    fn test_valid_percentage() {
        let valid_percentage = Percentage::new(85);
        assert_eq!(valid_percentage, Ok(Percentage(85)));
    }

    #[test]
    fn test_invalid_percentage() {
        let invalid_percentage = Percentage::new(150);
        assert!(invalid_percentage.is_err());
        assert_eq!(
            invalid_percentage.unwrap_err().to_string(),
            "Percent value must be between 0 and 100"
        );
    }

    #[test]
    fn test_metric_percent_display() {
        let category = Category::Cpu;
        let percentage = Percentage::new(50).unwrap();
        let metric = Metric::Percent(host(), category, percentage);
        assert_eq!(metric.to_string(), "test-CPU: 50%");
    }

    #[test]
    fn test_metric_used_display() {
        let category = Category::Memory;
        let metric = Metric::Used(host(), category, 4096, 8192);
        assert_eq!(metric.to_string(), "test-Memory: 4096/8192");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Cpu.to_string(), "CPU");
        assert_eq!(Category::Memory.to_string(), "Memory");
        assert_eq!(Category::Disk.to_string(), "Disk");
        assert_eq!(Category::Swap.to_string(), "Swap");
        assert_eq!(Category::Temperature.to_string(), "temperature");
    }

    #[test]
    fn test_percentage_ordering() {
        let p1 = Percentage::new(50).unwrap();
        let p2 = Percentage::new(75).unwrap();
        assert!(p1 < p2);
    }

    #[test]
    fn test_percentage_equality() {
        let p1 = Percentage::new(50).unwrap();
        let p2 = Percentage::new(50).unwrap();
        assert_eq!(p1, p2);
    }
}
