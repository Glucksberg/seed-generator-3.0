# GPU Seed Generator - Python Version

Python implementation of the BIP39 mnemonic seed generator with GPU acceleration.

## Features

- GPU acceleration via PyTorch (NVIDIA CUDA)
- Automatic resource throttling (80% CPU/GPU limit)
- Real-time monitoring and statistics
- Interactive GPU configuration
- Automatic PyTorch installation

## Quick Start

**Option 1 - Using batch file (easiest):**
```bash
run.bat
```

**Option 2 - Command line:**
```bash
python gpuseed3.py
```

## Installation

### 1. Install Python Dependencies
```bash
pip install -r requirements.txt
```

### 2. Install PyTorch with CUDA (for GPU acceleration)

**NVIDIA GPU:**
```bash
# CUDA 12.1
pip install torch --index-url https://download.pytorch.org/whl/cu121

# Or CUDA 11.8
pip install torch --index-url https://download.pytorch.org/whl/cu118
```

The program can also install PyTorch automatically on first run.

## Command Line Options

```bash
python gpuseed3.py [OPTIONS]
```

**Available options:**
- `--batch-size <N>` - Batch size for GPU processing (default: 2048)
- `--logfile <file>` - Log file name (default: mnemonics_log.txt)
- `--reset-config` - Reset GPU configuration

**Note:** `--threshold` and `--count` parameters are no longer used. The program is configured to:
- Search for seeds with < 46 characters
- Limit of 5 for 43-45 character seeds
- No limit for 42 character or less seeds (collects all)

## Requirements

**Mandatory:**
- Python 3.8+
- mnemonic (BIP39 library)
- colorama (terminal colors)

**Optional but recommended:**
- PyTorch with CUDA (for GPU acceleration)
- psutil (for CPU monitoring)
- pynvml (for NVIDIA GPU monitoring)

## Performance

With NVIDIA GPU:
- **Speed**: ~1,000,000 iterations/second
- **2 hours**: ~1-1.5 billion seeds tested

Without GPU (CPU only):
- **Speed**: ~30,000-50,000 iterations/second

## Configuration

GPU configuration is saved to `../gpuseed_config.json` and shared between Python and Rust versions.

To reconfigure GPU:
```bash
python gpuseed3.py --reset-config
```

## Output Files

- `mnemonics_log.txt` - Detailed log with all found seeds
- Default location: `python/` directory

## Stopping the Program

Press `q` and Enter to stop gracefully.

## Troubleshooting

**GPU not detected:**
- Ensure NVIDIA drivers are installed
- Install PyTorch with CUDA support
- Run `nvidia-smi` to verify GPU is accessible

**Low performance:**
- Increase `--batch-size` (try 4096 or 8192)
- Close other GPU-intensive applications
- Check resource monitor output for throttling

## See Also

- Main README: `../README.md`
- Rust version: `../rust/README.md`
