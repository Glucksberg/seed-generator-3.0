use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::System;

const MAX_USAGE_PERCENT: f64 = 0.80;
const MONITOR_INTERVAL: Duration = Duration::from_millis(500);

pub struct ResourceMonitor {
    stop_flag: Arc<AtomicBool>,
    throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    system: System,
}

impl ResourceMonitor {
    pub fn new(
        stop_flag: Arc<AtomicBool>,
        throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    ) -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        
        Self {
            stop_flag,
            throttle_data,
            system,
        }
    }
    
    pub fn run(mut self) {
        while !self.stop_flag.load(Ordering::Relaxed) {
            self.system.refresh_cpu_all();
            
            // Calculate CPU usage
            let cpu_count = self.system.cpus().len();
            let cpu_usage: f64 = if cpu_count > 0 {
                self.system
                    .cpus()
                    .iter()
                    .map(|cpu| cpu.cpu_usage() as f64 / 100.0)
                    .sum::<f64>() / cpu_count as f64
            } else {
                0.0
            };
            
            // Calculate throttle factor
            let cpu_throttle = if cpu_usage > MAX_USAGE_PERCENT {
                MAX_USAGE_PERCENT / cpu_usage
            } else {
                1.0
            };
            
            // GPU usage would be monitored here if GPU support is added
            let gpu_usage = 0.0;
            let gpu_throttle = 1.0;
            
            // Update throttle data
            let mut data = self.throttle_data.lock().unwrap();
            data.insert("cpu_usage".to_string(), cpu_usage);
            data.insert("gpu_usage".to_string(), gpu_usage);
            data.insert("cpu_throttle".to_string(), cpu_throttle);
            data.insert("gpu_throttle".to_string(), gpu_throttle);
            
            std::thread::sleep(MONITOR_INTERVAL);
        }
    }
}

