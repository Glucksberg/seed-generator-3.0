use bip39::{Language, Mnemonic};
use rand::RngCore;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct WorkerPool {
    num_workers: usize,
    batch_size: usize,
    threshold: usize,
    count_per_threshold: usize,
    use_gpu: bool,
    stop_flag: Arc<AtomicBool>,
    throttle_data: Arc<Mutex<HashMap<String, f64>>>,
    iterations: Arc<AtomicU64>,
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
        Self {
            num_workers,
            batch_size,
            threshold,
            count_per_threshold,
            use_gpu,
            stop_flag,
            throttle_data,
            iterations: Arc::new(AtomicU64::new(0)),
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
        _use_gpu: bool,
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
            
            // Generate entropy batch
            for _ in 0..adjusted_batch_size {
                if stop_flag.load(Ordering::Relaxed) {
                    return;
                }
                
                // Increment iteration counter
                iterations.fetch_add(1, Ordering::Relaxed);
                
                // Generate 16 bytes of entropy
                let mut entropy = [0u8; 16];
                rng.fill_bytes(&mut entropy);
                
                // Convert to mnemonic
                match Mnemonic::from_entropy_in(Language::English, &entropy) {
                    Ok(mnemonic) => {
                        let mnemonic_str = mnemonic.to_string();
                        let total_chars = mnemonic_str.replace(' ', "").len();
                        
                        if total_chars <= threshold {
                            // Lock both structures together to avoid race conditions
                            let mut counts = found_counts.lock().unwrap();
                            let count = counts.entry(total_chars).or_insert(0);
                            
                            // Double-check after acquiring lock
                            if *count < count_per_threshold {
                                // Check if we already have this mnemonic
                                let mut results_guard = results.lock().unwrap();
                                if !results_guard.iter().any(|(m, _)| m == &mnemonic_str) {
                                    results_guard.push((mnemonic_str.clone(), total_chars));
                                    *count += 1;
                                    
                                    println!("\nMnemonic: {}", mnemonic_str);
                                    println!("Total characters: {}", total_chars);
                                    
                                    // Check if we've reached the target for this threshold
                                    if *count >= count_per_threshold {
                                        // Release locks before checking if we should stop
                                        drop(results_guard);
                                        drop(counts);
                                        
                                        // Note: We don't stop automatically when finding enough
                                        // for one threshold, as we want to find mnemonics for
                                        // multiple different character counts. The user can
                                        // stop manually with Ctrl+C when satisfied.
                                    }
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
}
