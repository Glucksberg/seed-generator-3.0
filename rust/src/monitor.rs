use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::System;

#[cfg(feature = "gpu-monitoring")]
use nvml_wrapper::NVML;

const MAX_USAGE_PERCENT: f64 = 0.80;
const MONITOR_INTERVAL: Duration = Duration::from_millis(500);

pub struct ResourceMonitor {
    stop_flag: Arc<AtomicBool>,
    throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    system: System,
    gpu_monitor: Option<GpuMonitor>,
}

// GPU monitor using NVML for NVIDIA GPUs
struct GpuMonitor {
    #[cfg(feature = "gpu-monitoring")]
    nvml: NVML,
    #[cfg(feature = "gpu-monitoring")]
    device_index: u32,
}

impl GpuMonitor {
    #[cfg(feature = "gpu-monitoring")]
    fn new() -> Option<Self> {
        match NVML::init() {
            Ok(nvml) => {
                match nvml.device_count() {
                    Ok(count) if count > 0 => {
                        println!("GPU monitoring initialized (NVML)");
                        Some(Self {
                            nvml,
                            device_index: 0,
                        })
                    }
                    _ => {
                        eprintln!("No GPU devices found for monitoring");
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to initialize NVML: {:?}", e);
                None
            }
        }
    }
    
    #[cfg(not(feature = "gpu-monitoring"))]
    fn new() -> Option<Self> {
        None
    }
    
    #[cfg(feature = "gpu-monitoring")]
    fn get_usage(&self) -> f64 {
        match self.nvml.device_by_index(self.device_index) {
            Ok(device) => {
                match device.utilization_rates() {
                    Ok(utilization) => utilization.gpu as f64 / 100.0,
                    Err(_) => 0.0,
                }
            }
            Err(_) => 0.0,
        }
    }
    
    #[cfg(not(feature = "gpu-monitoring"))]
    fn get_usage(&self) -> f64 {
        0.0
    }
}

impl ResourceMonitor {
    pub fn new(
        stop_flag: Arc<AtomicBool>,
        throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    ) -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        
        let gpu_monitor = GpuMonitor::new();
        
        Self {
            stop_flag,
            throttle_data,
            system,
            gpu_monitor,
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
            
            // Get GPU usage (currently always 0.0 because GPU entropy generation not implemented)
            let gpu_usage = self.gpu_monitor.as_ref()
                .map(|m| m.get_usage())
                .unwrap_or(0.0);
            
            // Calculate GPU throttle factor
            let gpu_throttle = if gpu_usage > MAX_USAGE_PERCENT {
                MAX_USAGE_PERCENT / gpu_usage
            } else {
                1.0
            };
            
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

