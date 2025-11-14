#[cfg(feature = "gpu")]
#[macro_use]
extern crate rustacuda;

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

mod config;
mod gpu;
mod monitor;
mod worker;

use config::Config;
use monitor::ResourceMonitor;
use worker::WorkerPool;

#[derive(Parser, Debug)]
#[command(name = "gpuseed-rust")]
#[command(about = "High-performance BIP39 mnemonic generator with GPU support")]
struct Args {
    /// Character count threshold
    #[arg(long, default_value_t = 46)]
    threshold: usize,

    /// Number of mnemonics per character count
    #[arg(long, default_value_t = 5)]
    count: usize,

    /// Log file name
    #[arg(long, default_value = "mnemonics_log.txt")]
    logfile: String,

    /// Batch size for processing
    #[arg(long, default_value_t = 8192)]
    batch_size: usize,

    /// Output file for seeds list
    #[arg(long, default_value = "seeds_output.txt")]
    output: String,

    /// Reset GPU configuration
    #[arg(long)]
    reset_config: bool,
}

const CONFIG_FILE: &str = "../gpuseed_config.json";

fn main() {
    let args = Args::parse();
    
    // Validate inputs
    if args.threshold == 0 || args.threshold > 200 {
        eprintln!("Error: Threshold must be between 1 and 200");
        std::process::exit(1);
    }
    
    if args.count == 0 || args.count > 1000 {
        eprintln!("Error: Count must be between 1 and 1000");
        std::process::exit(1);
    }
    
    if args.batch_size == 0 || args.batch_size > 1_000_000 {
        eprintln!("Error: Batch size must be between 1 and 1,000,000");
        std::process::exit(1);
    }
    
    // Handle config reset
    if args.reset_config {
        if let Err(e) = fs::remove_file(CONFIG_FILE) {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("Warning: Could not delete configuration file: {}", e);
            }
        } else {
            println!("Configuration reset successfully.\n");
        }
    }
    
    // Load or create configuration
    let config = Config::load_or_create();
    
    // Determine GPU usage
    let use_gpu = if args.reset_config || config.is_none() {
        prompt_gpu_setup()
    } else {
        prompt_gpu_usage(&config.unwrap())
    };
    
    if !use_gpu {
        println!("\nRunning in CPU-only mode.");
    } else {
        println!("\nGPU mode enabled (configuration saved).");
    }
    
    // Initialize output file
    if let Err(e) = fs::File::create(&args.output) {
        eprintln!("Warning: Could not create output file: {}", e);
    }
    
    println!("\nStarting mnemonic generation...");
    println!("Looking for mnemonics with LESS than 46 characters");
    println!("  - 43-45 chars: limit of 5 per count");
    println!("  - 42 or less: NO LIMIT (collect all unique)");
    println!("Batch size: {}", args.batch_size);
    println!("Output file: {}", args.output);
    println!("GPU: {}", if use_gpu { "Enabled" } else { "Disabled" });
    
    // Start resource monitor
    let stop_flag = Arc::new(AtomicBool::new(false));
    let throttle_data = Arc::new(std::sync::Mutex::new(HashMap::new()));
    
    let monitor_handle = {
        let stop_flag = stop_flag.clone();
        let throttle_data = throttle_data.clone();
        std::thread::spawn(move || {
            ResourceMonitor::new(stop_flag, throttle_data).run();
        })
    };
    
    // Start worker pool
    let num_workers = num_cpus::get().max(1); // Ensure at least 1 worker
    println!("Starting {} worker threads...", num_workers);
    
    let worker_pool = WorkerPool::new(
        num_workers,
        args.batch_size,
        args.threshold,
        args.count,
        use_gpu,
        stop_flag.clone(),
        throttle_data.clone(),
    );
    
    let start_time = Instant::now();
    
    println!("Press Ctrl+C to stop...\n");
    println!("Resource limit: 80% (safety system active)");
    
    // Get iterations counter for status display
    let iterations_counter = worker_pool.get_iterations_counter();
    
    // Handle Ctrl+C
    let stop_flag_ctrlc = stop_flag.clone();
    ctrlc::set_handler(move || {
        println!("\n\nInterrupted by user. Shutting down gracefully...");
        stop_flag_ctrlc.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");
    
    // Run worker pool in a separate thread
    let worker_results = Arc::new(std::sync::Mutex::new(Vec::new()));
    let worker_results_clone = worker_results.clone();
    let worker_pool_for_thread = worker_pool;
    
    let worker_handle = std::thread::spawn(move || {
        let results = worker_pool_for_thread.run();
        *worker_results_clone.lock().unwrap() = results;
    });
    
    // Status display loop
    let status_interval = std::time::Duration::from_secs(5);
    let mut last_status_time = Instant::now();
    
    // Show initial status immediately
    display_status(&iterations_counter, &start_time, &throttle_data);
    
    while !stop_flag.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Check if worker thread finished
        if worker_handle.is_finished() {
            break;
        }
        
        // Display status every 5 seconds
        let now = Instant::now();
        if now.duration_since(last_status_time) >= status_interval {
            display_status(&iterations_counter, &start_time, &throttle_data);
            last_status_time = now;
        }
    }
    
    // Stop monitor and workers
    stop_flag.store(true, Ordering::Relaxed);

    // Give workers a moment to finish (they check stop_flag)
    let timeout = std::time::Duration::from_secs(2);
    let start_wait = std::time::Instant::now();
    while !worker_handle.is_finished() && start_wait.elapsed() < timeout {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Join worker thread first (wait for it to finish)
    let _ = worker_handle.join();

    // Join monitor (should finish quickly)
    monitor_handle.join().ok();

    // Get results - clone from Arc since try_unwrap may fail
    let results = {
        let results_guard = worker_results.lock().unwrap();
        results_guard.clone()
    };
    
    // Clear status line
    println!();
    
    // Save results
    save_results(&results, &args.logfile, &args.output, start_time);
    
    println!("\nGeneration complete!");
    println!("Total mnemonics found: {}", results.len());
    println!("Time elapsed: {:?}", start_time.elapsed());
}

fn display_status(
    iterations_counter: &Arc<AtomicU64>,
    start_time: &Instant,
    throttle_data: &Arc<std::sync::Mutex<HashMap<String, f64>>>,
) {
    let iterations = iterations_counter.load(Ordering::Relaxed);
    let elapsed = start_time.elapsed();
    // Calculate speed in iterations per second (same as Python)
    let speed = if elapsed.as_secs_f64() > 0.0 {
        iterations as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };
    
    // Get resource usage from throttle data
    let (cpu_usage, gpu_usage, cpu_throttle, gpu_throttle) = {
        let data = throttle_data.lock().unwrap();
        let cpu_usage = data.get("cpu_usage").copied().unwrap_or(0.0) * 100.0;
        let gpu_usage = data.get("gpu_usage").copied().unwrap_or(0.0) * 100.0;
        let cpu_throttle = data.get("cpu_throttle").copied().unwrap_or(1.0);
        let gpu_throttle = data.get("gpu_throttle").copied().unwrap_or(1.0);
        (cpu_usage, gpu_usage, cpu_throttle, gpu_throttle)
    };
    
    // Build throttle status string
    let throttle_status = if cpu_throttle < 1.0 || gpu_throttle < 1.0 {
        format!(" [THROTTLE: CPU={:.2}, GPU={:.2}]", cpu_throttle, gpu_throttle)
    } else {
        String::new()
    };
    
    // Format iterations with commas
    let iterations_str = format_number(iterations);
    
    // Print status line (overwrite previous line) - same format as Python
    print!("\rProcessed: {} ({:.0}/s) | CPU: {:.1}% | GPU: {:.1}%{}", 
        iterations_str, speed, cpu_usage, gpu_usage, throttle_status);
    io::stdout().flush().ok();
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result
}

fn prompt_gpu_setup() -> bool {
    println!("\n============================================================");
    println!("GPU Configuration Setup");
    println!("============================================================\n");
    
    println!("Do you want to use GPU acceleration? (much faster)");
    println!("1. Yes, use GPU");
    println!("2. No, use CPU only");
    print!("\nEnter your choice (1 or 2): ");
    
    if io::stdout().flush().is_err() {
        return false;
    }
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    match input.trim() {
        "1" => {
            println!("\nWhat type of GPU do you have?");
            println!("1. NVIDIA");
            println!("2. AMD/Intel (Windows DirectML)");
            print!("\nEnter your choice (1 or 2): ");
            
            if io::stdout().flush().is_err() {
                return false;
            }
            
            let mut gpu_input = String::new();
            if io::stdin().read_line(&mut gpu_input).is_err() {
                return false;
            }
            
            match gpu_input.trim() {
                "1" => {
                    println!("\nGPU acceleration enabled (NVIDIA).");
                    Config::save_gpu_type("nvidia");
                    true
                }
                "2" => {
                    println!("\nGPU acceleration enabled (AMD/Intel DirectML).");
                    Config::save_gpu_type("amd");
                    true
                }
                _ => {
                    println!("\nInvalid choice. Defaulting to NVIDIA.");
                    Config::save_gpu_type("nvidia");
                    true
                }
            }
        }
        _ => {
            println!("\nCPU-only mode selected.");
            Config::save_gpu_type("cpu");
            false
        }
    }
}

fn prompt_gpu_usage(_config: &Config) -> bool {
    println!("\nDo you want to use GPU for this session?");
    println!("1. Yes, use GPU");
    println!("2. No, use CPU only");
    print!("\nEnter your choice (1 or 2): ");
    
    if io::stdout().flush().is_err() {
        return false;
    }
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    input.trim() == "1"
}

fn save_results(
    results: &[(String, usize)],
    logfile: &str,
    output: &str,
    start_time: Instant,
) {
    let elapsed = start_time.elapsed();

    if results.is_empty() {
        println!("No seeds found matching criteria (< 46 characters).");
        return;
    }

    // Save to log file
    let mut log_content = String::new();
    for (mnemonic, chars) in results {
        log_content.push_str(&format!(
            "Mnemonic: {}\nTotal characters: {}\nTime elapsed: {:?}\n----------------------------------\n",
            mnemonic, chars, elapsed
        ));
    }
    match fs::write(logfile, &log_content) {
        Ok(_) => println!("Saved {} seeds to {}", results.len(), logfile),
        Err(e) => eprintln!("Error: Could not write to log file: {}", e),
    }

    // Save to output file (simple format)
    let mut output_content = String::new();
    for (mnemonic, chars) in results {
        output_content.push_str(&format!("{} {}\n", mnemonic, chars));
    }
    match fs::write(output, &output_content) {
        Ok(_) => println!("Saved {} seeds to {}", results.len(), output),
        Err(e) => eprintln!("Error: Could not write to output file: {}", e),
    }
}

