use config::Config;
use domain::{metric::Category, metric_service::MetricService, ports::MetricProcessor};
use outbound::{metric_reader::DummyMetricReader, metric_writer::DummyMetricWriter};

pub mod config;
pub mod domain;
pub mod outbound;


fn main() {
    let config = Config::from_env().expect("Error reading parameters.");
    if config.server_port.is_some() {
        println!("Config server_port={:?}", config.server_port);
    } else {
        let reader = DummyMetricReader;
        let writer = DummyMetricWriter;
        let service = MetricService::new(reader, writer); // Create an instance of DummyMetricService
        service.process_metrics(Category::Disk);
        service.process_metrics(Category::Cpu);
        service.process_metrics(Category::Memory);
    }
}
