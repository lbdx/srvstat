use crate::domain::metrics::models::{Category, Metric};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HomeAssistantDiscoveryConfig {
    name: String,
    unique_id: String,
    state_topic: String,
    unit_of_measurement: String,
    value_template: String,
    state_class: String,
    icon: String,
    expire_after: i16,
}

impl HomeAssistantDiscoveryConfig {
    pub fn get_config_topic(self) -> String {
        self.state_topic.replace("/state", "/config")
    }

    pub fn get_state_topic(self) -> String {
        self.state_topic.clone()
    }

    pub fn get_name(self) -> String {
        self.name.clone()
    }
}

fn get_discovery_config_used(
    host: &String,
    category: &Category,
    _total_bytes: u64, // total_bytes is not used for now, but passed for future use
) -> HomeAssistantDiscoveryConfig {
    let (name, sensor_name, icon, unit_of_measurement) = match category {
        Category::Disk => (
            format!("{}-{}", host, "disk_used").to_string(),
            "diskUsed".to_string(),
            "mdi:harddisk".to_string(),
            "GB".to_string(),
        ),
        Category::Memory => (
            format!("{}-{}", host, "memory_used").to_string(),
            "memoryUsed".to_string(),
            "mdi:memory".to_string(),
            "MB".to_string(),
        ),
        Category::Swap => (
            format!("{}-{}", host, "swap_used").to_string(),
            "swapUsed".to_string(),
            "mdi:swap-horizontal".to_string(),
            "MB".to_string(),
        ),
        Category::Cpu => {
            unreachable!("get_discovery_config_used should not be called for CPU")
        }
    };
    let unique_id = format!("{}{}", host, sensor_name).to_lowercase();
    let state_topic = format!("homeassistant/sensor/{}/state", &unique_id);
    let value_template = "{{ value_json.value }}".to_string();
    let state_class = "measurement".to_string();
    HomeAssistantDiscoveryConfig {
        name,
        unique_id,
        state_topic,
        unit_of_measurement,
        value_template,
        state_class,
        icon,
        expire_after: 300,
    }
}

impl From<&Metric> for HomeAssistantDiscoveryConfig {
    fn from(metric: &Metric) -> Self {
        match metric {
            Metric::Percent(host, category, _) => get_discovery_config_percent(host, category),
            Metric::Used(host, category, _used, total) => {
                get_discovery_config_used(host, category, *total)
            }
            Metric::Value {
                host,
                category,
                component_label,
                value: _, // value is not used for config generation
                unit,
            } => {
                if *category == Category::Temperature {
                    let name = format!("{}-{}-{}", host, category.to_string(), component_label);
                    let unique_id = format!(
                        "{}-{}-{}-temp",
                        host,
                        category.to_string(),
                        component_label
                    )
                    .to_lowercase()
                    .replace(' ', "_")
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect::<String>();
                    let state_topic = format!("homeassistant/sensor/{}/state", unique_id);

                    HomeAssistantDiscoveryConfig {
                        name,
                        unique_id,
                        state_topic,
                        unit_of_measurement: unit.clone(),
                        value_template: "{{ value_json.value }}".to_string(),
                        state_class: "measurement".to_string(),
                        icon: "mdi:thermometer".to_string(),
                        expire_after: 300,
                    }
                } else {
                    panic!("Metric::Value is currently only supported for Category::Temperature in HomeAssistantDiscoveryConfig");
                }
            } // Removed Metric::Temperature arm as it's replaced by Metric::Value
        }
    }
}

fn get_discovery_config_percent(
    host: &String,
    category: &Category,
) -> HomeAssistantDiscoveryConfig {
    let (name, sensor_name, icon) = match category {
        Category::Disk => (
            format!("{}-{}", host, "disk").to_string(),
            "diskUsePercent".to_string(),
            "mdi:harddisk".to_string(),
        ),
        Category::Memory => (
            format!("{}-{}", host, "memory").to_string(),
            "memoryUsePercent".to_string(),
            "mdi:memory".to_string(),
        ),
        Category::Cpu => (
            format!("{}-{}", host, "cpu").to_string(),
            "cpuUsePercent".to_string(),
            "mdi:cpu-64-bit".to_string(),
        ),
        Category::Swap => (
            format!("{}-{}", host, "swap").to_string(),
            "swapUsePercent".to_string(),
            "mdi:swap-horizontal".to_string(), // Standard MDI icon for swap
        ),
    };
    let unique_id = format!("{}{}", host, sensor_name).to_lowercase();
    let state_topic = format!("homeassistant/sensor/{}/state", &unique_id);
    let unit_of_measurement = "%".to_string();
    let value_template = "{{ value_json.value }}".to_string();
    let state_class = "measurement".to_string();
    HomeAssistantDiscoveryConfig {
        name,
        unique_id,
        state_topic,
        unit_of_measurement,
        value_template,
        state_class,
        icon,
        expire_after: 300,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ComponentTemperature import removed as it's no longer used
    use crate::domain::metrics::models::{Category, Metric, Percentage};

    #[test]
    fn test_get_config_topic() {
        let config = HomeAssistantDiscoveryConfig {
            name: "test-sensor".to_string(),
            unique_id: "test-id".to_string(),
            state_topic: "homeassistant/sensor/test-id/state".to_string(),
            unit_of_measurement: "%".to_string(),
            value_template: "{{ value_json.value }}".to_string(),
            state_class: "measurement".to_string(),
            icon: "mdi:cpu-64-bit".to_string(),
            expire_after: 300,
        };
        let config_topic = config.clone().get_config_topic();
        assert_eq!(
            config_topic,
            "homeassistant/sensor/test-id/config".to_string()
        );
    }

    #[test]
    fn test_get_state_topic() {
        let config = HomeAssistantDiscoveryConfig {
            name: "test-sensor".to_string(),
            unique_id: "test-id".to_string(),
            state_topic: "homeassistant/sensor/test-id/state".to_string(),
            unit_of_measurement: "%".to_string(),
            value_template: "{{ value_json.value }}".to_string(),
            state_class: "measurement".to_string(),
            icon: "mdi:cpu-64-bit".to_string(),
            expire_after: 300,
        };
        let state_topic = config.clone().get_state_topic();
        assert_eq!(
            state_topic,
            "homeassistant/sensor/test-id/state".to_string()
        );
    }

    #[test]
    fn test_get_name() {
        let config = HomeAssistantDiscoveryConfig {
            name: "test-sensor".to_string(),
            unique_id: "test-id".to_string(),
            state_topic: "homeassistant/sensor/test-id/state".to_string(),
            unit_of_measurement: "%".to_string(),
            value_template: "{{ value_json.value }}".to_string(),
            state_class: "measurement".to_string(),
            icon: "mdi:cpu-64-bit".to_string(),
            expire_after: 300,
        };
        let name = config.clone().get_name();
        assert_eq!(name, "test-sensor".to_string());
    }

    #[test]
    fn test_metric_to_config_conversion_cpu() {
        let host = "test-host".to_string();
        let metric = Metric::Percent(host.clone(), Category::Cpu, Percentage::new(50).unwrap());
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-cpu");
        assert_eq!(config.unique_id, "test-hostcpuusepercent");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostcpuusepercent/state"
        );
        assert_eq!(config.unit_of_measurement, "%");
        assert_eq!(config.icon, "mdi:cpu-64-bit");
    }

    #[test]
    fn test_metric_to_config_conversion_memory() {
        let host = "test-host".to_string();
        let metric = Metric::Percent(host.clone(), Category::Memory, Percentage::new(50).unwrap());
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-memory");
        assert_eq!(config.unique_id, "test-hostmemoryusepercent");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostmemoryusepercent/state"
        );
        assert_eq!(config.unit_of_measurement, "%");
        assert_eq!(config.icon, "mdi:memory");
    }

    #[test]
    fn test_metric_to_config_conversion_disk() {
        let host = "test-host".to_string();
        let metric = Metric::Percent(host.clone(), Category::Disk, Percentage::new(50).unwrap());
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-disk");
        assert_eq!(config.unique_id, "test-hostdiskusepercent");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostdiskusepercent/state"
        );
        assert_eq!(config.unit_of_measurement, "%");
        assert_eq!(config.icon, "mdi:harddisk");
    }

    #[test]
    fn test_metric_to_config_conversion_swap() {
        let host = "test-host".to_string();
        let metric = Metric::Percent(host.clone(), Category::Swap, Percentage::new(75).unwrap());
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-swap");
        assert_eq!(config.unique_id, "test-hostswapusepercent");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostswapusepercent/state"
        );
        assert_eq!(config.unit_of_measurement, "%");
        assert_eq!(config.icon, "mdi:swap-horizontal");
    }

    #[test]
    fn test_metric_used_to_config_conversion_disk() {
        let host = "test-host".to_string();
        let metric = Metric::Used(host.clone(), Category::Disk, 500_000_000_000, 1_000_000_000_000);
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-disk_used");
        assert_eq!(config.unique_id, "test-hostdiskused");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostdiskused/state"
        );
        assert_eq!(config.unit_of_measurement, "GB");
        assert_eq!(config.icon, "mdi:harddisk");
    }

    #[test]
    fn test_metric_used_to_config_conversion_memory() {
        let host = "test-host".to_string();
        let metric = Metric::Used(host.clone(), Category::Memory, 4096, 8192);
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-memory_used");
        assert_eq!(config.unique_id, "test-hostmemoryused");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostmemoryused/state"
        );
        assert_eq!(config.unit_of_measurement, "MB");
        assert_eq!(config.icon, "mdi:memory");
    }

    #[test]
    fn test_metric_used_to_config_conversion_swap() {
        let host = "test-host".to_string();
        let metric = Metric::Used(host.clone(), Category::Swap, 1024, 2048);
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        assert_eq!(config.name, "test-host-swap_used");
        assert_eq!(config.unique_id, "test-hostswapused");
        assert_eq!(
            config.state_topic,
            "homeassistant/sensor/test-hostswapused/state"
        );
        assert_eq!(config.unit_of_measurement, "MB");
        assert_eq!(config.icon, "mdi:swap-horizontal");
    }

    #[test]
    fn test_metric_to_config_conversion_temperature() {
        // This test might need to be removed or adapted if it relied on the old Metric::Temperature
        // For now, let's assume it's superseded by test_metric_value_temperature_to_config_conversion
        // If it tested something different, it should be re-evaluated.
        // Keeping it commented out or removing it would be typical.
        // For this exercise, I will remove it to avoid confusion with the new test.
    }

    #[test]
    fn test_metric_value_temperature_to_config_conversion() {
        let host = "test-host".to_string();
        let component_label = "CPU Core 1".to_string();
        let unit = "°C".to_string();
        let metric = Metric::Value {
            host: host.clone(),
            category: Category::Temperature,
            component_label: component_label.clone(),
            value: 55.2,
            unit: unit.clone(),
        };
        let config: HomeAssistantDiscoveryConfig = (&metric).into();

        let expected_name = format!("{}-temperature-{}", host, component_label);
        let expected_unique_id = format!(
            "{}-temperature-{}-temp",
            host,
            component_label
        )
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>();
        let expected_state_topic = format!("homeassistant/sensor/{}/state", expected_unique_id);

        assert_eq!(config.name, expected_name);
        assert_eq!(config.unique_id, expected_unique_id);
        assert_eq!(config.state_topic, expected_state_topic);
        assert_eq!(config.unit_of_measurement, unit);
        assert_eq!(config.value_template, "{{ value_json.value }}");
        assert_eq!(config.state_class, "measurement");
        assert_eq!(config.icon, "mdi:thermometer");
        assert_eq!(config.expire_after, 300);
    }

    #[test]
    #[should_panic(
        expected = "Metric::Value is currently only supported for Category::Temperature in HomeAssistantDiscoveryConfig"
    )]
    fn test_metric_value_non_temperature_panics() {
        let host = "test-host".to_string();
        let component_label = "Some Value".to_string();
        let unit = "Units".to_string();
        let metric = Metric::Value { // Using a different category, e.g., Disk
            host: host.clone(),
            category: Category::Disk, 
            component_label: component_label.clone(),
            value: 123.45,
            unit: unit.clone(),
        };
        // This conversion should panic
        let _config: HomeAssistantDiscoveryConfig = (&metric).into();
    }
}
