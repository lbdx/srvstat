use crate::domain::{
    ports::MetricReader,
};
use sysinfo::{Disks, System};
use crate::domain::metrics::models::{Category, Metric, Percentage};

pub struct DummyMetricReader;
impl MetricReader for DummyMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        Metric::Percent(category.clone(), Percentage::new(25).unwrap())
    }
    fn get_used(&self, category: &Category) -> Metric {
        Metric::Used(category.clone(), 25, 100)
    }
}

pub struct SystemMetricReader;

impl MetricReader for SystemMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        match category {
            Category::Cpu => {
                let mut sys = System::new_all();
                sys.refresh_cpu_usage();
                let cpuUsage = sys.global_cpu_usage() as u8;
                Metric::Percent(category.clone(), Percentage::new(cpuUsage).unwrap())
            }
            _ => {
                let used = self.get_used(category);
                match used {
                    Metric::Used(_, used_metric, total_metric) => {
                        // Calculate the usage percentage
                        let usage_percent = 1f64 / total_metric as f64 * used_metric as f64 * 100f64;
                        // Cast the percentage to u8 (after rounding)
                        let usage_percent_u8 = usage_percent.round() as u8;
                        let result = Percentage::new(usage_percent_u8);
                        Metric::Percent(category.clone(), result.expect(&format!("Erreur poucentage non valide : {}", usage_percent_u8).to_string()))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn get_used(&self, category: &Category) -> Metric {
        match category {
            Category::Disk => {
                // Initialize the system info struct
                let mut sys = System::new_all();
                
                // Refresh system data to ensure we get the latest info
                sys.refresh_all();

                // Get the first disk
                let disks = Disks::new_with_refreshed_list();
                let disk = disks.first();
                let total_space = disk.unwrap().total_space();
                let available_space = disk.unwrap().available_space();

                // Calculate used space
                let used_space = total_space - available_space;

                Metric::Used(category.clone(), used_space, total_space)
            }
            Category::Memory => {
                 // Initialize the system info struct
                 let mut sys = System::new_all();

                 // Refresh system data to ensure we get the latest info
                 sys.refresh_memory();
                 Metric::Used(category.clone(), sys.used_memory(), sys.total_memory())
            },
            Category::Cpu => {
                todo!()
            },
        }
    }

}