use super::metric::Category;
use super::ports::{MetricProcessor, MetricReader, MetricWriter};

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
        self.writer.write(self.reader.get_used(&category));
    }
}
