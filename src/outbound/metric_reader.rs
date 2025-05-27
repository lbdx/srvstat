use crate::domain::metrics::models::{Category, Metric, Percentage};
use crate::domain::ports::MetricReader;
use sysinfo::{Disks, System};

pub struct DummyMetricReader;
impl MetricReader for DummyMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        Metric::Percent(
            "tux".to_string(),
            category.clone(),
            Percentage::new(25).unwrap(),
        )
    }
    fn get_used(&self, category: &Category) -> Metric {
        Metric::Used("tux".to_string(), category.clone(), 25, 100)
    }
}

pub struct SystemMetricReader;

impl MetricReader for SystemMetricReader {
    fn get_percent(&self, category: &Category) -> Metric {
        let mut sys = System::new_all();
        let host = System::host_name().unwrap();
        match category {
            Category::Cpu => {
                sys.refresh_cpu_usage();
                let cpu_usage = sys.global_cpu_usage() as u8;
                Metric::Percent(host, category.clone(), Percentage::new(cpu_usage).unwrap())
            }
            _ => {
                let used = self.get_used(category);
                match used {
                    Metric::Used(host, _, used_metric, total_metric) => {
                        // Calculate the usage percentage
                        let usage_percent =
                            1f64 / total_metric as f64 * used_metric as f64 * 100f64;
                        // Cast the percentage to u8 (after rounding)
                        let usage_percent_u8 = usage_percent.round() as u8;
                        let result = Percentage::new(usage_percent_u8);
                        Metric::Percent(
                            host.clone(),
                            category.clone(),
                            result.unwrap_or_else(|_| {
                                panic!(
                                    "{}",
                                    format!("Erreur poucentage non valide : {}", usage_percent_u8)
                                        .to_string()
                                )
                            }),
                        )
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn get_used(&self, category: &Category) -> Metric {
        // Initialize the system info struct
        let mut sys = System::new_all();
        let host = System::host_name().unwrap();
        match category {
            Category::Disk => {
                // Refresh system data to ensure we get the latest info
                sys.refresh_all();

                // Get the first disk
                let disks = Disks::new_with_refreshed_list();
                let disk = disks.first();
                let total_space = disk.unwrap().total_space();
                let available_space = disk.unwrap().available_space();

                // Calculate used space
                let used_space = total_space - available_space;

                Metric::Used(host, category.clone(), used_space, total_space)
            }
            Category::Memory => {
                // Refresh system data to ensure we get the latest info
                sys.refresh_memory();
                Metric::Used(
                    host,
                    category.clone(),
                    sys.used_memory(),
                    sys.total_memory(),
                )
            }
            Category::Cpu => {
                // error no used metric for cpu
                eprintln!("Error: no used metric for cpu");
                Metric::Used(host, category.clone(), 0, 0)
            }
            Category::Swap => {
                // sys and host are already defined in the outer scope of this match
                sys.refresh_memory(); // refresh memory info
                let used_swap = sys.used_swap(); // get used swap
                let total_swap = sys.total_swap(); // get total swap
                Metric::Used(
                    host,
                    category.clone(),
                    used_swap,
                    total_swap,
                )
            }
        }
    }
}
