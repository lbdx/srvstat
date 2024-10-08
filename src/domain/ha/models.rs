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

impl From<&Metric> for HomeAssistantDiscoveryConfig {
    fn from(metric: &Metric) -> Self {
        match metric {
            Metric::Percent(host, category, _) => get_discovery_config_percent(host, category),
            Metric::Used(_, _, _, _) => todo!(),
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
}
