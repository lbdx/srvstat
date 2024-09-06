use std::fmt;

use thiserror::Error;

/// Represents different metrics that can be tracked.
#[derive(Debug, PartialEq)]
pub enum Metric {
    /// A percentage-based metric (0-100) for a specific category.
    Percent(String, Category, Percentage),
    /// A used/total value pair for a specific category, such as memory usage.
    Used(String, Category, u64, u64),
}

/// Represents the different categories of resources that can be measured.
#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    Disk,
    Memory,
    Cpu,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Metric::Percent(host, category, Percentage(value)) => write!(f, "{}-{}: {}%", host, category, value),
            Metric::Used(host, category, used, total) => write!(f, "{}-{}: {}/{}", host, category, used, total),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Disk => write!(f, "Disk"),
            Category::Memory => write!(f, "Memory"),
            Category::Cpu => write!(f, "CPU"),
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
    use super::*;

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
        let metric = Metric::Percent(category, percentage);
        assert_eq!(metric.to_string(), "CPU: 50%");
    }

    #[test]
    fn test_metric_used_display() {
        let category = Category::Memory;
        let metric = Metric::Used(category, 4096, 8192);
        assert_eq!(metric.to_string(), "Memory: 4096/8192");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Cpu.to_string(), "CPU");
        assert_eq!(Category::Memory.to_string(), "Memory");
        assert_eq!(Category::Disk.to_string(), "Disk");
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
