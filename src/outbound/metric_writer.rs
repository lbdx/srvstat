use crate::domain::ports::MetricWriter;

pub struct DummyMetricWriter;

impl MetricWriter for DummyMetricWriter {
    fn write(&self, metric: crate::domain::metric::Metric) {
        println!("{:?}", metric);
    }
}

pub struct MqttMetricWriter;

impl MetricWriter for MqttMetricWriter {
    fn write(&self, metric: crate::domain::metric::Metric) {
        todo!()
    }
}