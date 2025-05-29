use crate::domain::metrics::models::{Category, Metric}; // ComponentTemperature removed
use crate::domain::ports::{MetricProcessor, MetricReader, MetricWriter};
use sysinfo::Components; // Ensure this is present

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
            let host = self.reader.get_host();
            let components = Components::new_with_refreshed_list();
            let mut temps_found = false;

            for component in components.list() {
                if let Some(temp_value) = component.temperature() {
                    temps_found = true; // A temperature value was found and will be processed
                    let temp_metric = Metric::Value {
                        host: host.clone(),
                        category: Category::Temperature,
                        component_label: component.label().to_string(),
                        value: temp_value,
                        unit: "°C".to_string(),
                    };
                    self.writer.write(temp_metric);
                }
                // If component.temperature() is None, we simply skip this component.
            }

            if !temps_found { // This now correctly means no valid temperatures were processed
                eprintln!("Warning: No temperature components found or sysinfo couldn't read them. Temperature metrics will not be published.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::metrics::models::{Category, Metric, Percentage}; // Added Percentage for get_percent
    use std::cell::RefCell;
    use std::sync::Arc; // Not strictly needed with RefCell for single-threaded tests, but good practice for shared concepts

    // 1. MockMetricWriter
    #[derive(Clone)] // Clone is needed if MetricService takes writer by value and clones it, or if we clone it.
    struct MockMetricWriter {
        metrics_written: Arc<RefCell<Vec<Metric>>>,
    }

    impl MockMetricWriter {
        fn new() -> Self {
            Self {
                metrics_written: Arc::new(RefCell::new(Vec::new())),
            }
        }
    }

    impl MetricWriter for MockMetricWriter {
        fn write(&self, metric: Metric) {
            self.metrics_written.borrow_mut().push(metric); // No need to clone metric if it's owned
        }
    }

    // 2. MockMetricReader
    struct MockMetricReader;

    impl MetricReader for MockMetricReader {
        fn get_host(&self) -> String {
            "test-host".to_string()
        }

        // get_percent is called unconditionally by process_metrics
        fn get_percent(&self, _category: &Category) -> Metric {
            // Return a dummy Metric::Percent as it's expected by process_metrics
            Metric::Percent("test-host".to_string(), _category.clone(), Percentage::new(0).unwrap())
        }

        fn get_used(&self, _category: &Category) -> Metric {
            unimplemented!("Not needed for this test if process_metrics doesn't call it for Temperature")
        }
    }

    // 3. Test function
    #[test]
    fn test_process_temperature_metrics() {
        let mock_reader = MockMetricReader;
        let mock_writer = MockMetricWriter::new();

        let metric_service = MetricService::new(mock_reader, mock_writer.clone()); // Clone writer if service stores it

        metric_service.process_metrics(Category::Temperature);

        let metrics = mock_writer.metrics_written.borrow();
        println!("Found {} metrics in total after processing.", metrics.len());

        // Filter for only Metric::Value, as get_percent also writes a metric
        let temp_metrics: Vec<&Metric> = metrics.iter().filter(|m| matches!(m, Metric::Value {..})).collect();
        println!("Found {} temperature (Metric::Value) metrics.", temp_metrics.len());


        if !temp_metrics.is_empty() {
            for metric_value in temp_metrics {
                if let Metric::Value { host, category, component_label, value, unit } = metric_value {
                    println!("Metric: {:?}", metric_value);
                    assert_eq!(*category, Category::Temperature);
                    assert_eq!(host, "test-host");
                    assert_eq!(unit, "°C");
                    assert!(!component_label.is_empty(), "Component label should not be empty");
                    // Basic plausibility for temperature value (not NaN, not excessively out of range)
                    assert!(!value.is_nan(), "Temperature value should not be NaN");
                    // Depending on the environment, temps could be low (e.g. unpowered components)
                    // or higher. A very broad check:
                    assert!(*value > -100.0 && *value < 200.0, "Temperature value seems implausible: {}", value);
                } else {
                    panic!("Expected Metric::Value, got something else after filtering.");
                }
            }
        } else {
            // This branch executes if sysinfo found no temperature components on the test machine
            // The initial Metric::Percent from get_percent() might still be there in `metrics` (before filter)
            println!("No temperature components (Metric::Value) found in this test environment. Warning message to stderr is expected from the service if sysinfo returned no components.");
        }
        
        // Check for the initial Metric::Percent that is always written
        let percent_metrics: Vec<&Metric> = metrics.iter().filter(|m| matches!(m, Metric::Percent {..})).collect();
        assert_eq!(percent_metrics.len(), 1, "Expected one Metric::Percent to always be written");
        if let Some(Metric::Percent(host, category, _)) = percent_metrics.first() {
            assert_eq!(host, "test-host");
            assert_eq!(*category, Category::Temperature); // get_percent is called with Category::Temperature
        } else {
            panic!("Expected a Metric::Percent as the first metric written");
        }
    }
}
