use crate::domain::metrics::models::{Category, ComponentTemperature, Metric};
use crate::domain::ports::{MetricProcessor, MetricReader, MetricWriter};
use sysinfo::Components;

// Generic service for reading and writing metrics
#[derive(Debug, Clone)]
pub struct MetricService<R, W>
where
    R: MetricReader,
    W: MetricWriter,
{
    reader: R,
    writer: W,
}

impl<R, W> MetricService<R, W>
where
    R: MetricReader,
    W: MetricWriter,
{
    // Constructors
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl<R, W> MetricProcessor for MetricService<R, W>
where
    R: MetricReader,
    W: MetricWriter,
{
    // read and write metric for a category (disk, cpu, ...)
    fn process_metrics(&self, category: Category) {
        self.writer.write(self.reader.get_percent(&category));
        //if category != Category::Cpu {
        //     self.writer.write(self.reader.get_used(&category));
        // }

        if category == Category::Temperature {
            // Collect temperature metrics
            let components = Components::new_with_refreshed_list();
            let mut temp_metrics: Vec<ComponentTemperature> = Vec::new();

            for component in components.list() {
                temp_metrics.push(ComponentTemperature {
                    label: component.label().to_string(),
                    temperature: component.temperature(),
                    unit: "Celsius".to_string(),
                });
            }

            if !temp_metrics.is_empty() {
                self.writer.write(Metric::Temperature(self.reader.get_host(), temp_metrics));
            } else {
                eprintln!("Warning: No temperature components found or sysinfo couldn't read them. Temperature metrics will not be published.");
            }
        }
    }
}
