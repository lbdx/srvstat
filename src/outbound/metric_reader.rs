use crate::domain::{
    metric::{Category, Metric, Percentage},
    ports::MetricReader,
};
pub struct DummyMetricReader;
impl MetricReader for DummyMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        Metric::Percent(category.clone(), Percentage::new(25).unwrap())
    }
    fn get_used(&self, category: &Category) -> Metric {
        Metric::Used(category.clone(), 25, 100)
    }
}
