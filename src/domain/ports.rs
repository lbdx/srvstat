use crate::domain::metrics::models::{Category, Metric};

pub trait MetricProcessor {
    fn process_metrics(&self, category: Category);
}

pub trait MetricReader {
    fn get_percent(&self, category: &Category) -> Metric;
    fn get_used(&self, category: &Category) -> Metric;
    fn get_host(&self) -> String;
}

pub trait MetricWriter {
    fn write(&self, metric: Metric);
}
