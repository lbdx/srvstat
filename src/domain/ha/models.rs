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
            "Disque".to_string(),
            "diskUsePercent".to_string(),
            "mdi:harddisk".to_string(),
        ),
        Category::Memory => (
            "Mémoire".to_string(),
            "memoryUsePercent".to_string(),
            "mdi:memory".to_string(),
        ),
        Category::Cpu => (
            "Cpu".to_string(),
            "cpuUsePercent".to_string(),
            "mdi:cpu-64-bit".to_string(),
        ),
    };
    let unique_id = format!("{}{}", host, sensor_name);
    let state_topic = format!("homeassistant/sensor/{}/state", &unique_id);
    let unit_of_measurement = "%".to_string();
    let value_template = "{{ value_json.value }}".to_string();
    let state_class = "measurement".to_string();
    HomeAssistantDiscoveryConfig {
        name: name,
        unique_id,
        state_topic,
        unit_of_measurement,
        value_template,
        state_class,
        icon,
    }
}
