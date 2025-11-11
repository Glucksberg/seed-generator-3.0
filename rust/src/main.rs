use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
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
    }
    
    // Initialize output file
    if let Err(e) = fs::File::create(&args.output) {
        eprintln!("Warning: Could not create output file: {}", e);
    }
    
    println!("\nStarting mnemonic generation...");
    println!("Threshold: {} characters", args.threshold);
    println!("Count per threshold: {}", args.count);
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
    
    // Handle Ctrl+C
    let stop_flag_ctrlc = stop_flag.clone();
    ctrlc::set_handler(move || {
        println!("\n\nInterrupted by user. Shutting down gracefully...");
        stop_flag_ctrlc.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");
    
    let results = worker_pool.run();
    
    // Stop monitor
    stop_flag.store(true, Ordering::Relaxed);
    monitor_handle.join().ok();
    
    // Save results
    save_results(&results, &args.logfile, &args.output, start_time);
    
    println!("\nGeneration complete!");
    println!("Total mnemonics found: {}", results.len());
    println!("Time elapsed: {:?}", start_time.elapsed());
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

fn prompt_gpu_usage(config: &Config) -> bool {
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
    
    // Save to log file
    if !results.is_empty() {
        let mut log_content = String::new();
        for (mnemonic, chars) in results {
            log_content.push_str(&format!(
                "Mnemonic: {}\nTotal characters: {}\nTime elapsed: {:?}\n----------------------------------\n",
                mnemonic, chars, elapsed
            ));
        }
        if let Err(e) = fs::write(logfile, log_content) {
            eprintln!("Warning: Could not write to log file: {}", e);
        }
        
        // Save to output file (simple format)
        let mut output_content = String::new();
        for (mnemonic, chars) in results {
            output_content.push_str(&format!("{} {}\n", mnemonic, chars));
        }
        if let Err(e) = fs::write(output, output_content) {
            eprintln!("Warning: Could not write to output file: {}", e);
        }
    }
}

