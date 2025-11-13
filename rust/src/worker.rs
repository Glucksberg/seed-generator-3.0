use bip39::{Language, Mnemonic};
use rand::RngCore;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::gpu::GpuContext;

pub struct WorkerPool {
    num_workers: usize,
    batch_size: usize,
    threshold: usize,
    count_per_threshold: usize,
    use_gpu: bool,
    stop_flag: Arc<AtomicBool>,
    throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    iterations: Arc<AtomicU64>,
    gpu_context: Option<Arc<GpuContext>>,
}

impl WorkerPool {
    pub fn new(
        num_workers: usize,
        batch_size: usize,
        threshold: usize,
        count_per_threshold: usize,
        use_gpu: bool,
        stop_flag: Arc<AtomicBool>,
        throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    ) -> Self {
        // Initialize GPU context if GPU mode is enabled
        // Note: Each worker thread will need to create its own GPU context
        // because CUDA contexts are not thread-safe
        let gpu_context = if use_gpu {
            let ctx = GpuContext::new();
            if ctx.is_available() {
                Some(Arc::new(ctx))
            } else {
                None
            }
        } else {
            None
        };
        
        Self {
            num_workers,
            batch_size,
            threshold,
            count_per_threshold,
            use_gpu,
            stop_flag,
            throttle_data,
            iterations: Arc::new(AtomicU64::new(0)),
            gpu_context,
        }
    }
    
    pub fn run(&self) -> Vec<(String, usize)> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let found_counts: Arc<Mutex<HashMap<usize, usize>>> = Arc::new(Mutex::new(HashMap::new()));
        
        let handles: Vec<_> = (0..self.num_workers)
            .map(|_| {
                let stop_flag = self.stop_flag.clone();
                let throttle_data = self.throttle_data.clone();
                let results = results.clone();
                let found_counts = found_counts.clone();
                let iterations = self.iterations.clone();
                let batch_size = self.batch_size;
                let threshold = self.threshold;
                let count_per_threshold = self.count_per_threshold;
                let use_gpu = self.use_gpu;
                let gpu_context = self.gpu_context.clone();
                
                thread::spawn(move || {
                    Self::worker_loop(
                        stop_flag,
                        throttle_data,
                        results,
                        found_counts,
                        iterations,
                        batch_size,
                        threshold,
                        count_per_threshold,
                        use_gpu,
                        gpu_context,
                    );
                })
            })
            .collect();
        
        // Wait for all workers
        for handle in handles {
            handle.join().ok();
        }
        
        // Safely unwrap results
        Arc::try_unwrap(results)
            .ok()
            .and_then(|m| m.into_inner().ok())
            .unwrap_or_default()
    }
    
    fn worker_loop(
        stop_flag: Arc<AtomicBool>,
        throttle_data: Arc<Mutex<HashMap<String, f64>>>,
        results: Arc<Mutex<Vec<(String, usize)>>>,
        found_counts: Arc<Mutex<HashMap<usize, usize>>>,
        iterations: Arc<AtomicU64>,
        batch_size: usize,
        threshold: usize,
        count_per_threshold: usize,
        use_gpu: bool,
        gpu_context: Option<Arc<GpuContext>>,
    ) {
        let mut rng = rand::thread_rng();
        
        loop {
            // Check if we should stop
            if stop_flag.load(Ordering::Relaxed) {
                break;
            }
            
            // Check throttle factor
            let throttle_factor = {
                let data = throttle_data.lock().unwrap();
                let cpu_throttle = data.get("cpu_throttle").copied().unwrap_or(1.0);
                let gpu_throttle = data.get("gpu_throttle").copied().unwrap_or(1.0);
                cpu_throttle.min(gpu_throttle)
            };
            
            // Adjust batch size based on throttle
            let adjusted_batch_size = std::cmp::max(1, (batch_size as f64 * throttle_factor) as usize);
            
            // Try to use GPU if available, otherwise fallback to CPU
            let entropy_batch = if use_gpu {
                if let Some(ref gpu_ctx) = gpu_context {
                    // Try GPU generation - log attempt (first time only)
                    static GPU_ATTEMPT_LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                    if !GPU_ATTEMPT_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                        eprintln!("[GPU] Attempting to generate entropy on GPU (batch size: {})", adjusted_batch_size);
                    }
                    
                    match gpu_ctx.generate_entropy_batch(adjusted_batch_size) {
                        Ok(batch) => {
                            // GPU generation succeeded - log first time only
                            static GPU_SUCCESS_LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                            if !GPU_SUCCESS_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                                eprintln!("[GPU] Successfully generating entropy on GPU (batch size: {})", adjusted_batch_size);
                            }
                            Some(batch)
                        }
                        Err(e) => {
                            // GPU generation failed, fallback to CPU
                            // Only log once to avoid spam
                            static GPU_ERROR_LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                            if !GPU_ERROR_LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                                eprintln!("[GPU] ERROR: GPU generation failed: {}. Using CPU fallback.", e);
                            }
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };
            
            // Generate entropy batch (GPU or CPU)
            let entropies: Vec<[u8; 16]> = if let Some(gpu_batch) = entropy_batch {
                gpu_batch
            } else {
                // CPU fallback
                let mut batch = Vec::with_capacity(adjusted_batch_size);
                for _ in 0..adjusted_batch_size {
                    let mut entropy = [0u8; 16];
                    rng.fill_bytes(&mut entropy);
                    batch.push(entropy);
                }
                batch
            };
            
            // Process each entropy in the batch
            for entropy in entropies {
                if stop_flag.load(Ordering::Relaxed) {
                    return;
                }
                
                // Increment iteration counter
                iterations.fetch_add(1, Ordering::Relaxed);
                
                // Convert to mnemonic
                match Mnemonic::from_entropy_in(Language::English, &entropy) {
                    Ok(mnemonic) => {
                        let mnemonic_str = mnemonic.to_string();
                        let total_chars = mnemonic_str.replace(' ', "").len();

                        // Only collect seeds with less than 46 characters
                        // <= 42 chars: no limit (collect all unique)
                        // 43-45 chars: limit of 5 per character count
                        if total_chars < 46 {
                            // Lock both structures together to avoid race conditions
                            let mut counts = found_counts.lock().unwrap();
                            let count = counts.entry(total_chars).or_insert(0);

                            // Determine if we should add based on character count
                            let should_add = if total_chars <= 42 {
                                // No limit for 42 or less
                                true
                            } else {
                                // Limit of 5 for 43-45 characters
                                *count < 5
                            };

                            if should_add {
                                // Check if we already have this mnemonic
                                let mut results_guard = results.lock().unwrap();
                                if !results_guard.iter().any(|(m, _)| m == &mnemonic_str) {
                                    results_guard.push((mnemonic_str.clone(), total_chars));
                                    *count += 1;

                                    println!("\nMnemonic: {}", mnemonic_str);
                                    println!("Total characters: {}", total_chars);
                                }
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
            
            // Small delay to prevent CPU spinning
            if throttle_factor < 1.0 {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
    
    pub fn get_iterations(&self) -> u64 {
        self.iterations.load(Ordering::Relaxed)
    }
    
    pub fn get_iterations_counter(&self) -> Arc<AtomicU64> {
        self.iterations.clone()
    }
    
    pub fn cleanup(&self) {
        // Cleanup GPU contexts if GPU was used
        if let Some(ref gpu_ctx) = self.gpu_context {
            gpu_ctx.cleanup();
        }
    }
}
