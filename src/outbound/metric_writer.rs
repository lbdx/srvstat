use crate::domain::ha::models::HomeAssistantDiscoveryConfig;
use crate::domain::metrics::models::{Metric, Percentage};
use crate::domain::ports::MetricWriter;
use paho_mqtt as mqtt;
use paho_mqtt::{Client, QOS_0};
use std::process;
use std::time::Duration;

pub struct DummyMetricWriter;

impl MetricWriter for DummyMetricWriter {
    fn write(&self, metric: Metric) {
        println!("{:?}", metric);
    }
}

#[derive(Clone)]
pub struct MqttMetricWriter {
    client: Client,
}

impl MqttMetricWriter {
    pub fn new(broker: &str) -> Self {
        // Create a client & define connect options
        let client = Client::new(broker).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            process::exit(1);
        });

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();

        // Connect and wait for it to complete or fail
        if let Err(e) = client.connect(conn_opts) {
            println!("Unable to connect:\n\t{:?}", e);
            process::exit(1);
        }
        MqttMetricWriter { client }
    }

    fn publish_autodiscovery_config(&self, config: &HomeAssistantDiscoveryConfig) {
        let discovery_payload = serde_json::to_string(&config.clone()).unwrap();
        let discovery_topic = config.clone().get_config_topic();
        println!("config topic = {}", discovery_topic);
        println!("config payload = {}", discovery_payload);
        let msg = mqtt::Message::new(discovery_topic, discovery_payload, QOS_0);
        let tok = self.client.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
        }
    }

    fn publish_metric_value(self, config: HomeAssistantDiscoveryConfig, val: String) {
        let state_topic = config.get_state_topic();
        let payload = serde_json::json!({
            "value": val
        });
        let payload_str = serde_json::to_string(&payload).unwrap();
        println!("state topic = {}", &state_topic);
        println!("config payload = {}", &payload_str);
        let msg = mqtt::Message::new(state_topic, payload_str, QOS_0);
        let tok = self.client.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
        }
    }

}

impl MetricWriter for MqttMetricWriter {
    fn write(&self, metric: Metric) {
        let config = HomeAssistantDiscoveryConfig::from(&metric);
        // Publish Home Assistant autodiscovery config
        self.publish_autodiscovery_config(&config);
        // Publish actual metric value
        let val: String = match metric {
            Metric::Percent(_, _, percent) => {
                let Percentage(val) = percent;
                val.to_string()
            }
            Metric::Used(_, _, _, _) => {
                "".to_string()
            }
        };
        self.clone().publish_metric_value(config, val);
    }
}
