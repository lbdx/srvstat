use crate::domain::metrics::models::Metric;
use crate::domain::ports::MetricWriter;

pub struct DummyMetricWriter;

impl MetricWriter for DummyMetricWriter {
    fn write(&self, metric: Metric) {
        println!("{:?}", metric);
    }
}

pub struct MqttMetricWriter;

impl MetricWriter for MqttMetricWriter {
    fn write(&self, metric: Metric) {
        todo!()
    }
}