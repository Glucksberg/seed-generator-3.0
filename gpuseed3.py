import torch
from mnemonic import Mnemonic
import multiprocessing
import time
from datetime import timedelta
from colorama import Fore, Style, init
import sys
import argparse
import atexit

# Initialize colorama
init(autoreset=True)

# Constants
NUM_STREAMS = 4  # Number of CUDA streams for parallel processing
QUEUE_MAX_SIZE = 50000  # Increased queue size for better buffering
WORKER_DELAY = 0.01  # Base delay in workers
QUEUE_WARNING_THRESHOLD = 0.9  # Slow down workers if queue is 90% full
BATCH_SIZE_MIN = 512  # Minimum batch size

def generate_entropy_gpu(batch_size=4096):
    """
    Generate random entropy using GPU (if available) or CPU.
    """
    if torch.cuda.is_available():
        device = torch.device('cuda')
        streams = [torch.cuda.Stream() for _ in range(NUM_STREAMS)]
        entropies = []
        
        for stream in streams:
            with torch.cuda.stream(stream):
                entropy = torch.randint(0, 256, (batch_size, 16), device=device, dtype=torch.uint8)
                entropies.append(entropy)
        
        torch.cuda.synchronize()
        combined_entropy = torch.cat(entropies)
        result = combined_entropy.cpu().numpy()
        del combined_entropy, entropies  # Free GPU memory
        torch.cuda.empty_cache()  # Clear unused memory
        return result
    else:
        return torch.randint(0, 256, (batch_size, 16)).numpy()

def worker(queue, stop_event, batch_size):
    """
    Worker process to generate mnemonics and add them to the queue.
    """
    mnemo = Mnemonic("english")
    current_batch_size = batch_size
    consecutive_full_count = 0
    
    while not stop_event.is_set():
        try:
            # Dynamic batch size adjustment
            queue_size = queue.qsize()
            queue_ratio = queue_size / QUEUE_MAX_SIZE
            
            if queue_ratio > QUEUE_WARNING_THRESHOLD:
                consecutive_full_count += 1
                current_batch_size = max(BATCH_SIZE_MIN, current_batch_size // 2)
                time.sleep(WORKER_DELAY * consecutive_full_count)  # Progressive backoff
            else:
                consecutive_full_count = 0
                current_batch_size = min(batch_size, current_batch_size * 2)

            entropies = generate_entropy_gpu(current_batch_size)
            
            for entropy in entropies:
                if stop_event.is_set():
                    break
                try:
                    mnemonic = mnemo.to_mnemonic(entropy.tobytes())
                    total_chars = len(''.join(mnemonic.split()))
                    if not queue.full():  # Check before attempting to put
                        queue.put_nowait((mnemonic, total_chars))
                    else:
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
                        stop_event.set()
            else:
                import select
                if select.select([sys.stdin], [], [], 0)[0]:
                    line = sys.stdin.readline()
                    if 'q' in line.lower():
                        stop_event.set()
        except Exception:
            pass
        time.sleep(1)

def collector(queue, stop_event, character_threshold, mnemonics_per_count, log_file_name, start_time):
    """
    Collect and log mnemonics that meet the character threshold.
    """
    total_iterations = 0
    results_dict = {}
    last_status_time = time.time()
    status_interval = 5  # Status update every 5 seconds
    
    print("Collector started.")
    print(f"CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"Using GPU: {torch.cuda.get_device_name(0)}")
    print(f"Logging results to: {log_file_name}")
    print(f"Looking for mnemonics with {character_threshold} characters or less")
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
                print(f"\rProcessed: {total_iterations:,} ({speed:.0f}/s)", end="")
                last_status_time = current_time

            if total_chars <= character_threshold:
                if total_chars not in results_dict:
                    results_dict[total_chars] = set()

                if len(results_dict[total_chars]) < mnemonics_per_count:
                    if mnemonic not in results_dict[total_chars]:
                        results_dict[total_chars].add(mnemonic)
                        elapsed_time = time.time() - start_time
                        hms_time = str(timedelta(seconds=int(elapsed_time)))
                        print("\n")  # Clear status line
                        print_output(mnemonic, total_chars, total_iterations, hms_time)
                        log_result(mnemonic, total_chars, total_iterations, hms_time, log_file_name)

        except multiprocessing.queues.Empty:
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

def cleanup(stop_event, workers, listener_process):
    """
    Cleanup function to terminate all processes gracefully.
    """
    stop_event.set()
    for p in workers:
        p.terminate()
        p.join()
    listener_process.terminate()
    listener_process.join()
    print("All processes terminated.")

if __name__ == '__main__':
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description='Mnemonic Generator Script')
    parser.add_argument('--threshold', type=int, default=45, help='Character count threshold')
    parser.add_argument('--count', type=int, default=5, help='Number of mnemonics per character count')
    parser.add_argument('--logfile', type=str, default='mnemonics_log.txt', help='Log file name')
    parser.add_argument('--batch-size', type=int, default=2048, help='Batch size for GPU processing')
    args = parser.parse_args()

    # Initialize variables
    character_threshold = args.threshold
    mnemonics_per_count = args.count
    log_file_name = args.logfile
    batch_size = args.batch_size

    start_time = time.time()
    stop_event = multiprocessing.Event()
    queue = multiprocessing.Queue(maxsize=QUEUE_MAX_SIZE)

    # Start keyboard listener process
    listener_process = multiprocessing.Process(target=keyboard_listener, args=(stop_event,))
    listener_process.daemon = True
    listener_process.start()

    # Start worker processes
    num_processes = multiprocessing.cpu_count()
    workers = []
    for _ in range(num_processes):
        p = multiprocessing.Process(target=worker, args=(queue, stop_event, batch_size))
        workers.append(p)
        p.start()

    # Register cleanup function
    atexit.register(cleanup, stop_event, workers, listener_process)

    # Start collector process
    collector(queue, stop_event, character_threshold, mnemonics_per_count, log_file_name, start_time)