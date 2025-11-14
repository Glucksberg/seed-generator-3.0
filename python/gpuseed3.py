import torch
from mnemonic import Mnemonic
import multiprocessing
import time
from datetime import timedelta
from colorama import Fore, Style, init
import sys
import argparse
import atexit
import queue as queue_module  # stdlib queue for exception compatibility
import os
import json
import subprocess

# Optional monitoring libraries for resource usage control
try:
    import psutil
    PSUTIL_AVAILABLE = True
except ImportError:
    PSUTIL_AVAILABLE = False

# Optional NVIDIA GPU monitoring
try:
    import pynvml
    if torch.cuda.is_available():
        pynvml.nvmlInit()
        NVML_AVAILABLE = True
    else:
        NVML_AVAILABLE = False
except Exception:
    NVML_AVAILABLE = False

# Optional DirectML (AMD/Intel on Windows) support
# If available, we'll use it when CUDA is not available
try:
    import torch_directml  # type: ignore
    DML_AVAILABLE = True
    DML_DEVICE = torch_directml.device()
except Exception:
    DML_AVAILABLE = False
    DML_DEVICE = None

# Initialize colorama
# Force ANSI conversion on Windows and keep color codes (no strip) to ensure colored output
init(autoreset=True, convert=True, strip=False)

# Constants
NUM_STREAMS = 4  # Number of CUDA streams for parallel processing
QUEUE_MAX_SIZE = 50000  # Increased queue size for better buffering
WORKER_DELAY = 0.01  # Base delay in workers
QUEUE_WARNING_THRESHOLD = 0.9  # Slow down workers if queue is 90% full
BATCH_SIZE_MIN = 512  # Minimum batch size

# Resource usage limits (safety system)
MAX_USAGE_PERCENT = 0.80  # Maximum allowed usage: 80%
MONITOR_INTERVAL = 0.5  # Check resource usage every 0.5 seconds
THROTTLE_DELAY_BASE = 0.05  # Base delay when throttling is active

# Configuration file path
CONFIG_FILE = '../gpuseed_config.json'  # Relative to python/ directory

def load_config():
    """
    Load user configuration from file.
    Returns dict with configuration or None if file doesn't exist.
    """
    config_path = os.path.join(os.path.dirname(__file__), CONFIG_FILE)
    if os.path.exists(config_path):
        try:
            with open(config_path, 'r') as f:
                return json.load(f)
        except Exception:
            return None
    return None

def save_config(config):
    """
    Save user configuration to file.
    """
    config_path = os.path.join(os.path.dirname(__file__), CONFIG_FILE)
    try:
        with open(config_path, 'w') as f:
            json.dump(config, f, indent=2)
        return True
    except Exception as e:
        print(f"Warning: Could not save configuration: {e}")
        return False

def check_torch_installed():
    """
    Check if PyTorch is installed and what version (CUDA/DirectML/CPU).
    Returns tuple: (is_installed, gpu_type)
    gpu_type can be: 'cuda', 'directml', 'cpu', or None
    """
    try:
        import torch
        if torch.cuda.is_available():
            return True, 'cuda'
        try:
            import torch_directml
            if torch_directml.is_available():
                return True, 'directml'
        except ImportError:
            pass
        return True, 'cpu'
    except ImportError:
        return False, None

def install_pytorch(gpu_type):
    """
    Install PyTorch based on GPU type.
    gpu_type: 'nvidia' or 'amd'
    Returns True if successful, False otherwise.
    """
    print(f"\nInstalling PyTorch for {gpu_type.upper()} GPU...")
    print("This may take a few minutes. Please wait...\n")
    
    try:
        if gpu_type.lower() == 'nvidia':
            # Try CUDA 12.1 first, fallback to 11.8
            commands = [
                ['pip', 'install', '--upgrade', '--index-url', 'https://download.pytorch.org/whl/cu121', 'torch'],
                ['pip', 'install', '--upgrade', '--index-url', 'https://download.pytorch.org/whl/cu118', 'torch']
            ]
        else:
            # AMD/Intel DirectML
            commands = [
                ['pip', 'install', '--upgrade', 'torch-directml']
            ]
        
        for cmd in commands:
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode == 0:
                print(f"PyTorch installed successfully!")
                return True
        
        print("Failed to install PyTorch. Please install manually.")
        return False
    except Exception as e:
        print(f"Error installing PyTorch: {e}")
        return False

def prompt_gpu_setup():
    """
    Interactive prompt for GPU setup on first run.
    Returns tuple: (use_gpu: bool, gpu_type: str or None)
    """
    print("\n" + "="*60)
    print("GPU Configuration Setup")
    print("="*60)
    print("\nDo you want to use GPU acceleration? (much faster)")
    print("1. Yes, use GPU")
    print("2. No, use CPU only")
    
    while True:
        choice = input("\nEnter your choice (1 or 2): ").strip()
        if choice == '1':
            print("\nWhat type of GPU do you have?")
            print("1. NVIDIA")
            print("2. AMD/Intel (Windows DirectML)")
            
            while True:
                gpu_choice = input("\nEnter your choice (1 or 2): ").strip()
                if gpu_choice == '1':
                    gpu_type = 'nvidia'
                    break
                elif gpu_choice == '2':
                    gpu_type = 'amd'
                    break
                else:
                    print("Invalid choice. Please enter 1 or 2.")
            
            # Check if PyTorch is already installed
            is_installed, installed_type = check_torch_installed()
            
            if not is_installed:
                if install_pytorch(gpu_type):
                    save_config({'gpu_type': gpu_type})
                    return True, gpu_type
                else:
                    print("Failed to install PyTorch. Using CPU mode.")
                    save_config({'gpu_type': 'cpu'})
                    return False, None
            else:
                # PyTorch is installed, verify it matches GPU type
                if gpu_type == 'nvidia' and installed_type == 'cuda':
                    print(f"\nPyTorch is already configured for NVIDIA GPU.")
                    save_config({'gpu_type': gpu_type})
                    return True, gpu_type
                elif gpu_type == 'amd' and installed_type == 'directml':
                    print(f"\nPyTorch is already configured for AMD/Intel GPU.")
                    save_config({'gpu_type': gpu_type})
                    return True, gpu_type
                else:
                    print(f"\nPyTorch is installed but configured for {installed_type}.")
                    print("You may need to reinstall PyTorch for your GPU type.")
                    save_config({'gpu_type': gpu_type})
                    return True, gpu_type
            
        elif choice == '2':
            save_config({'gpu_type': 'cpu'})
            return False, None
        else:
            print("Invalid choice. Please enter 1 or 2.")

def prompt_gpu_usage(config):
    """
    Prompt user if they want to use GPU in this session.
    Uses saved configuration for GPU type.
    Returns tuple: (use_gpu: bool, gpu_type: str or None)
    """
    gpu_type = config.get('gpu_type')
    
    if gpu_type and gpu_type != 'cpu':
        print(f"\nGPU configured: {gpu_type.upper()}")
        print("Do you want to use GPU acceleration in this session?")
        print("1. Yes, use GPU")
        print("2. No, use CPU only")
        
        while True:
            choice = input("\nEnter your choice (1 or 2): ").strip()
            if choice == '1':
                # Verify PyTorch is still installed and working
                is_installed, installed_type = check_torch_installed()
                if not is_installed:
                    print("PyTorch not found. Installing...")
                    if install_pytorch(gpu_type):
                        return True, gpu_type
                    else:
                        print("Failed to install. Using CPU mode.")
                        return False, None
                
                # Check if GPU is actually available
                if gpu_type == 'nvidia':
                    try:
                        import torch
                        if not torch.cuda.is_available():
                            print("Warning: CUDA is not available. Check your NVIDIA drivers.")
                            print("Falling back to CPU mode.")
                            return False, None
                    except ImportError:
                        print("PyTorch not installed. Using CPU mode.")
                        return False, None
                
                return True, gpu_type
            elif choice == '2':
                return False, None
            else:
                print("Invalid choice. Please enter 1 or 2.")
    else:
        # No GPU configured, ask if they want to set it up
        print("\nNo GPU configuration found.")
        return prompt_gpu_setup()

def get_cpu_usage():
    """
    Get current CPU usage percentage.
    Returns 0.0 if psutil is not available.
    """
    if not PSUTIL_AVAILABLE:
        return 0.0
    try:
        return psutil.cpu_percent(interval=0.1) / 100.0
    except Exception:
        return 0.0

def get_gpu_usage():
    """
    Get current GPU usage percentage.
    Returns 0.0 if monitoring is not available.
    """
    if torch.cuda.is_available():
        # Try NVML first (most accurate)
        try:
            import pynvml
            handle = pynvml.nvmlDeviceGetHandleByIndex(0)
            utilization = pynvml.nvmlDeviceGetUtilizationRates(handle)
            return utilization.gpu / 100.0
        except Exception:
            # Fallback: try to estimate from CUDA memory usage
            try:
                # Simple heuristic: if memory is being used, assume some GPU activity
                # This is not perfect but better than nothing
                memory_used = torch.cuda.memory_allocated() / torch.cuda.max_memory_allocated() if torch.cuda.max_memory_allocated() > 0 else 0.0
                return min(memory_used * 1.2, 1.0)  # Rough estimate
            except Exception:
                return 0.0
    elif DML_AVAILABLE:
        # DirectML doesn't have direct monitoring, use time-based throttling
        return 0.0
    else:
        return 0.0

def generate_entropy_gpu(batch_size=4096, throttle_factor=1.0, use_gpu=True):
    """
    Generate random entropy using the best available accelerator:
    - CUDA (NVIDIA) with streams for parallelism
    - DirectML (AMD/Intel on Windows) if installed
    - CPU as a fallback
    
    Args:
        batch_size: Size of batch to generate
        throttle_factor: Multiplier to reduce batch size when throttling (0.0 to 1.0)
    """
    # Apply throttling by reducing batch size
    adjusted_batch_size = max(BATCH_SIZE_MIN, int(batch_size * throttle_factor))
    
    # Respect user's GPU choice
    if not use_gpu:
        # Force CPU mode
        return torch.randint(0, 256, (adjusted_batch_size, 16), dtype=torch.uint8).numpy()
    
    if torch.cuda.is_available():
        device = torch.device('cuda')
        # Reduce number of streams if throttling
        num_streams = max(1, int(NUM_STREAMS * throttle_factor))
        streams = [torch.cuda.Stream() for _ in range(num_streams)]
        entropies = []
        
        for stream in streams:
            with torch.cuda.stream(stream):
                entropy = torch.randint(0, 256, (adjusted_batch_size, 16), device=device, dtype=torch.uint8)
                entropies.append(entropy)
        
        torch.cuda.synchronize()
        combined_entropy = torch.cat(entropies)
        result = combined_entropy.cpu().numpy()
        del combined_entropy, entropies  # Free GPU memory
        torch.cuda.empty_cache()  # Clear unused memory
        return result
    elif DML_AVAILABLE:
        # DirectML path: generate directly on DML device and move to CPU
        # No streams available on DML; a single batch is efficient
        entropy = torch.randint(0, 256, (adjusted_batch_size, 16), device=DML_DEVICE, dtype=torch.uint8)
        result = entropy.cpu().numpy()
        del entropy
        return result
    else:
        # Use uint8 on CPU as well, to produce 16 bytes of entropy per item
        # This matches BIP39 expected entropy sizes (e.g., 16 bytes -> 12 words)
        return torch.randint(0, 256, (adjusted_batch_size, 16), dtype=torch.uint8).numpy()

def resource_monitor(stop_event, throttle_data):
    """
    Monitor CPU and GPU usage and update throttle factors.
    Runs in a separate process to continuously monitor resources.
    """
    # Initialize NVML in this process if available (needed for multiprocessing)
    if torch.cuda.is_available():
        try:
            import pynvml
            pynvml.nvmlInit()
        except Exception:
            pass  # Will use fallback method
    
    while not stop_event.is_set():
        try:
            cpu_usage = get_cpu_usage()
            gpu_usage = get_gpu_usage()
            
            # Calculate throttle factors (1.0 = no throttling, 0.0 = maximum throttling)
            cpu_throttle = 1.0
            gpu_throttle = 1.0
            
            if cpu_usage > MAX_USAGE_PERCENT:
                # Reduce throttle factor proportionally when over limit
                # Example: if at 90% and limit is 80%, reduce to ~0.89
                cpu_throttle = MAX_USAGE_PERCENT / max(cpu_usage, MAX_USAGE_PERCENT + 0.01)
            
            if gpu_usage > MAX_USAGE_PERCENT:
                # Same logic for GPU
                gpu_throttle = MAX_USAGE_PERCENT / max(gpu_usage, MAX_USAGE_PERCENT + 0.01)
            
            # Update shared throttle data
            throttle_data['cpu_throttle'] = cpu_throttle
            throttle_data['gpu_throttle'] = gpu_throttle
            throttle_data['cpu_usage'] = cpu_usage
            throttle_data['gpu_usage'] = gpu_usage
            
        except Exception as e:
            # On error, disable throttling to avoid stopping work
            throttle_data['cpu_throttle'] = 1.0
            throttle_data['gpu_throttle'] = 1.0
        
        time.sleep(MONITOR_INTERVAL)

def worker(queue, stop_event, batch_size, throttle_data, use_gpu=True):
    """
    Worker process to generate mnemonics and add them to the queue.
    Now includes resource-aware throttling.
    """
    # Info: indicate worker startup for diagnostics
    try:
        import os
        print(f"Worker started (PID: {os.getpid()})")  # Startup message
    except Exception:
        pass
    mnemo = Mnemonic("english")
    current_batch_size = batch_size
    consecutive_full_count = 0
    last_throttle_check = time.time()
    
    while not stop_event.is_set():
        try:
            # Check throttle factors periodically
            current_time = time.time()
            if current_time - last_throttle_check >= MONITOR_INTERVAL:
                cpu_throttle = throttle_data.get('cpu_throttle', 1.0)
                gpu_throttle = throttle_data.get('gpu_throttle', 1.0)
                # Use the more restrictive throttle factor
                throttle_factor = min(cpu_throttle, gpu_throttle)
                last_throttle_check = current_time
            else:
                throttle_factor = min(
                    throttle_data.get('cpu_throttle', 1.0),
                    throttle_data.get('gpu_throttle', 1.0)
                )
            
            # Dynamic batch size adjustment based on queue
            # Windows: queue.qsize() may not be implemented; fall back safely
            try:
                queue_size = queue.qsize()
                queue_ratio = queue_size / QUEUE_MAX_SIZE
            except (NotImplementedError, AttributeError, OSError):
                # Assume queue is not near full if qsize is unavailable
                queue_ratio = 0.0  # Safe default to keep producing
            
            if queue_ratio > QUEUE_WARNING_THRESHOLD:
                consecutive_full_count += 1
                current_batch_size = max(BATCH_SIZE_MIN, current_batch_size // 2)
                time.sleep(WORKER_DELAY * consecutive_full_count)  # Progressive backoff
            else:
                consecutive_full_count = 0
                current_batch_size = min(batch_size, current_batch_size * 2)

            # Apply resource throttling
            if throttle_factor < 1.0:
                # Add delay when throttling is active
                time.sleep(THROTTLE_DELAY_BASE * (1.0 - throttle_factor))

            entropies = generate_entropy_gpu(current_batch_size, throttle_factor, use_gpu)
            
            for entropy in entropies:
                if stop_event.is_set():
                    break
                try:
                    mnemonic = mnemo.to_mnemonic(entropy.tobytes())
                    total_chars = len(''.join(mnemonic.split()))
                    # Robust non-blocking enqueue; avoids unreliable full() on Windows
                    try:
                        queue.put_nowait((mnemonic, total_chars))
                    except queue_module.Full:
                        time.sleep(WORKER_DELAY)
                        break
                except Exception:
                    continue

        except Exception as e:
            print(f"Worker error: {e}")
            time.sleep(WORKER_DELAY)

def keyboard_listener(stop_event):
    """
    Listen for 'q' keypress to stop the script gracefully.
    """
    print("Press 'q' and Enter to stop...")
    while not stop_event.is_set():
        try:
            if sys.platform == 'win32':
                import msvcrt
                if msvcrt.kbhit():
                    key = msvcrt.getch()
                    if key.lower() == b'q':
                        # Debug: indicate stop was requested
                        print("\nStop signal received (q). Shutting down...")  # Added feedback on stop
                        stop_event.set()
            else:
                import select
                if select.select([sys.stdin], [], [], 0)[0]:
                    line = sys.stdin.readline()
                    if 'q' in line.lower():
                        # Debug: indicate stop was requested
                        print("\nStop signal received (q). Shutting down...")  # Added feedback on stop
                        stop_event.set()
        except Exception:
            pass
        time.sleep(1)

def collector(queue, stop_event, character_threshold, mnemonics_per_count, log_file_name, start_time, throttle_data):
    """
    Collect and log mnemonics that meet the character threshold.
    Now includes resource usage display.
    """
    total_iterations = 0
    results_dict = {}
    last_status_time = time.time()
    status_interval = 5  # Status update every 5 seconds
    empty_queue_count = 0  # Track consecutive empty polls
    
    print("Collector started.")
    print(f"CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"Using GPU: {torch.cuda.get_device_name(0)}")
    else:
        # Inform runtime device when CUDA is not available
        if DML_AVAILABLE:
            print("Using DirectML device for entropy generation")
        else:
            print("Using CPU for entropy generation")  # Added clarity on execution device
    
    # Display resource monitoring status
    if PSUTIL_AVAILABLE:
        print("CPU monitoring: Enabled")
    else:
        print("CPU monitoring: Limited (psutil not available)")
    
    if NVML_AVAILABLE:
        print("GPU monitoring: Enabled (NVML)")
    elif torch.cuda.is_available():
        print("GPU monitoring: Basic (CUDA memory-based)")
    else:
        print("GPU monitoring: Not available")
    
    print(f"Resource limit: {MAX_USAGE_PERCENT*100:.0f}% (safety system active)")
    print(f"Logging results to: {log_file_name}")
    print(f"Looking for mnemonics with LESS than 46 characters")
    print(f"  - 43-45 chars: limit of 5 per count")
    print(f"  - 42 or less: NO LIMIT (collect all unique)")
    print("Press 'q' and Enter to stop...")

    while not stop_event.is_set():
        try:
            mnemonic, total_chars = queue.get(timeout=0.1)  # Reduced timeout
            total_iterations += 1

            # Print status update periodically
            current_time = time.time()
            if current_time - last_status_time >= status_interval:
                elapsed = current_time - start_time
                speed = total_iterations / elapsed
                cpu_usage = throttle_data.get('cpu_usage', 0.0) * 100
                gpu_usage = throttle_data.get('gpu_usage', 0.0) * 100
                cpu_throttle = throttle_data.get('cpu_throttle', 1.0)
                gpu_throttle = throttle_data.get('gpu_throttle', 1.0)
                
                # Show throttling status
                throttle_status = ""
                if cpu_throttle < 1.0 or gpu_throttle < 1.0:
                    throttle_status = f" [THROTTLE: CPU={cpu_throttle:.2f}, GPU={gpu_throttle:.2f}]"
                
                print(f"\rProcessed: {total_iterations:,} ({speed:.0f}/s) | CPU: {cpu_usage:.1f}% | GPU: {gpu_usage:.1f}%{throttle_status}", end="")
                last_status_time = current_time

            # Only collect seeds with less than 46 characters
            # <= 42 chars: no limit (collect all unique)
            # 43-45 chars: limit of 5 per character count
            if total_chars < 46:
                if total_chars not in results_dict:
                    results_dict[total_chars] = set()

                # Check if we should add this seed
                should_add = False
                if total_chars <= 42:
                    # No limit for 42 or less
                    should_add = mnemonic not in results_dict[total_chars]
                else:
                    # Limit of 5 for 43-45 characters
                    should_add = len(results_dict[total_chars]) < 5 and mnemonic not in results_dict[total_chars]

                if should_add:
                    results_dict[total_chars].add(mnemonic)
                    elapsed_time = time.time() - start_time
                    hms_time = str(timedelta(seconds=int(elapsed_time)))
                    print("\n")  # Clear status line
                    print_output(mnemonic, total_chars, total_iterations, hms_time)
                    log_result(mnemonic, total_chars, total_iterations, hms_time, log_file_name)

        except (multiprocessing.queues.Empty, queue_module.Empty):
            # If queue stays empty for a while, provide a hint it is still running
            empty_queue_count += 1
            if empty_queue_count % int(5 / 0.1) == 0:  # about every 5 seconds
                print("\nWaiting for data from workers...", end="")
            continue

    print("\nCollector terminating.")

def print_output(mnemonic, total_chars, total_iterations, hms_time):
    """
    Print mnemonic details to the console with colored output.
    """
    print(Fore.GREEN + Style.BRIGHT + f"Mnemonic: {mnemonic}")
    print(Fore.CYAN + Style.BRIGHT + f"Total characters: {total_chars}")
    print(Fore.YELLOW + Style.BRIGHT + f"Total iterations: {total_iterations}")
    print(Fore.MAGENTA + Style.BRIGHT + f"Time elapsed: {hms_time}")
    print(Fore.WHITE + Style.BRIGHT + "----------------------------------")

def log_result(mnemonic, total_chars, total_iterations, hms_time, log_file_name):
    """
    Log mnemonic details to a file.
    """
    log_entry = (
        f"Mnemonic: {mnemonic}\n"
        f"Total characters: {total_chars}\n"
        f"Total iterations: {total_iterations}\n"
        f"Time elapsed: {hms_time}\n"
        "----------------------------------\n"
    )
    with open(log_file_name, "a") as f:
        f.write(log_entry)

def cleanup(stop_event, workers, listener_process, monitor_process=None):
    """
    Cleanup function to terminate all processes gracefully.
    """
    stop_event.set()
    for p in workers:
        p.terminate()
        p.join()
    listener_process.terminate()
    listener_process.join()
    if monitor_process is not None:
        monitor_process.terminate()
        monitor_process.join()
    print("All processes terminated.")

if __name__ == '__main__':
    # Windows safety: ensure child processes can start correctly when using spawn
    multiprocessing.freeze_support()  # Added for Windows stability
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description='Mnemonic Generator Script')
    parser.add_argument('--threshold', type=int, default=46, help='Character count threshold')
    parser.add_argument('--count', type=int, default=5, help='Number of mnemonics per character count')
    parser.add_argument('--logfile', type=str, default='mnemonics_log.txt', help='Log file name')
    parser.add_argument('--batch-size', type=int, default=2048, help='Batch size for GPU processing')
    parser.add_argument('--reset-config', action='store_true', help='Reset GPU configuration')
    args = parser.parse_args()
    
    # Handle config reset
    if args.reset_config:
        config_path = os.path.join(os.path.dirname(__file__), CONFIG_FILE)
        if os.path.exists(config_path):
            try:
                os.remove(config_path)
                print("Configuration reset successfully.\n")
            except Exception as e:
                print(f"Warning: Could not delete configuration file: {e}\n")
        else:
            print("No configuration file found.\n")
    
    # Load or create configuration
    config = load_config()
    
    # Determine GPU usage
    if args.reset_config or config is None:
        use_gpu, gpu_type = prompt_gpu_setup()
    else:
        use_gpu, gpu_type = prompt_gpu_usage(config)
    
    # Set CUDA_VISIBLE_DEVICES if CPU-only mode
    if not use_gpu:
        os.environ['CUDA_VISIBLE_DEVICES'] = ''
        print("\nRunning in CPU-only mode.")
    else:
        gpu_display = gpu_type.upper() if gpu_type else "GPU"
        print(f"\nGPU acceleration enabled ({gpu_display}).")

    # Initialize variables
    character_threshold = args.threshold
    mnemonics_per_count = args.count
    log_file_name = args.logfile
    batch_size = args.batch_size

    start_time = time.time()
    stop_event = multiprocessing.Event()
    queue = multiprocessing.Queue(maxsize=QUEUE_MAX_SIZE)
    
    # Create shared dictionary for throttle data
    manager = multiprocessing.Manager()
    throttle_data = manager.dict({
        'cpu_throttle': 1.0,
        'gpu_throttle': 1.0,
        'cpu_usage': 0.0,
        'gpu_usage': 0.0
    })

    # Start resource monitor process
    monitor_process = multiprocessing.Process(target=resource_monitor, args=(stop_event, throttle_data))
    monitor_process.daemon = True
    monitor_process.start()
    print("Resource monitor started (safety system: 80% limit)")

    # Start keyboard listener process
    listener_process = multiprocessing.Process(target=keyboard_listener, args=(stop_event,))
    listener_process.daemon = True
    listener_process.start()

    # Start worker processes
    num_processes = multiprocessing.cpu_count()
    workers = []
    print(f"Starting {num_processes} worker processes...")
    for i in range(num_processes):
        try:
            p = multiprocessing.Process(target=worker, args=(queue, stop_event, batch_size, throttle_data, use_gpu))
            workers.append(p)
            p.start()
            print(f"Worker {i+1} launch requested")
        except Exception as e:
            print(f"Failed to start worker {i+1}: {e}")
    # Give workers a brief moment to initialize (helpful on Windows spawn)
    time.sleep(0.5)

    # Register cleanup function
    atexit.register(cleanup, stop_event, workers, listener_process, monitor_process)

    # Start collector process
    collector(queue, stop_event, character_threshold, mnemonics_per_count, log_file_name, start_time, throttle_data)