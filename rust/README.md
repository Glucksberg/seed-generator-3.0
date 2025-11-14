# GPU Seed Generator - Rust Version

High-performance Rust implementation with direct CUDA kernel integration.

## Features

- **Maximum Performance**: 10-50x faster than Python
- **GPU Acceleration**: Direct CUDA kernel with cuRAND
- **Thread-Safe**: Guaranteed by Rust compiler
- **Resource Monitoring**: 80% CPU/GPU safety limit
- **Parallel Processing**: Utilizes all CPU cores efficiently

## Quick Start

**Option 1 - Using batch file (easiest):**
```bash
run.bat
```

**Option 2 - Command line:**
```bash
cargo run --release --features gpu
```

**Option 3 - Direct execution (after compilation):**
```bash
.\target\release\gpuseed-rust.exe
```

## Requirements

### Mandatory
- **Rust 1.70+** with Cargo
- **CUDA Toolkit 11.8 or 12.x**
- **Visual Studio Build Tools 2019+** (or Visual Studio 2022+) with C++ component
- **NVIDIA GPU** with compute capability 6.0+

### Installation Links
- Rust: https://rustup.rs/
- CUDA Toolkit: https://developer.nvidia.com/cuda-downloads
- Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/ (scroll to "Tools for Visual Studio")

## Installation & Setup

### 1. Install Prerequisites

Install in this order:
1. Visual Studio Build Tools (with C++ workload)
2. CUDA Toolkit
3. Rust (via rustup)

### 2. Verify Installation

```powershell
# Check CUDA
nvcc --version

# Check Rust
rustc --version
cargo --version

# Check Visual Studio C++ compiler
cl.exe
```

### 3. Setup Environment (Windows PowerShell)

**Option A - Automatic setup script:**
```powershell
cd rust
.\setup_env.ps1
```

**Option B - Manual setup:**
```powershell
# Add Cargo to PATH (if not already)
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Set CUDA paths
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.0"
```

### 4. Compile with GPU Support

```bash
cargo build --release --features gpu
```

This will:
- Compile CUDA kernels automatically
- Generate optimized executable
- Take 2-5 minutes on first build

## Usage

```bash
# Run directly (after compilation)
.\target\release\gpuseed-rust.exe

# Or use cargo (checks for changes and recompiles if needed)
cargo run --release --features gpu

# Or use the batch file
run.bat
```

### Command Line Options

```bash
gpuseed-rust.exe [OPTIONS]
```

**Available options:**
- `--batch-size <N>` - Batch size for processing (default: 8192)
- `--logfile <file>` - Detailed log file (default: mnemonics_log.txt)
- `--output <file>` - Simple output file (default: seeds_output.txt)
- `--reset-config` - Reset GPU configuration

**Note:** `--threshold` and `--count` are no longer used. Program is configured to:
- Search for seeds with < 46 characters
- Limit of 5 for 43-45 character seeds
- No limit for ≤ 42 character seeds (collects all)

## Performance

With NVIDIA GPU:
- **Speed**: ~1,500,000 iterations/second
- **2 hours**: ~2 billion seeds tested

Compilation time:
- **First build**: 2-5 minutes
- **Incremental**: 5-20 seconds
- **No changes**: 0.05 seconds (instant)

## Why Visual Studio?

Rust uses the Microsoft C++ compiler (`cl.exe`) on Windows for linking. The Visual Studio Build Tools provide this compiler. You don't need the full Visual Studio IDE, just the Build Tools with C++ support.

## GPU Implementation Details

The Rust version uses:
- **RustaCUDA**: Low-level CUDA bindings
- **cuRAND**: NVIDIA's random number generator
- **Custom kernels**: Compiled at build time via `build.rs`
- **Thread-local contexts**: Each worker thread has its own GPU context

This provides significantly better performance than PyTorch abstraction used in Python version.

## Project Structure

```
rust/
├── Cargo.toml          # Project configuration & dependencies
├── build.rs            # Build script (compiles CUDA kernels)
├── test_kernel.cu      # CUDA kernel source code
├── run.bat             # Quick launch script
├── src/
│   ├── main.rs         # Entry point & CLI
│   ├── config.rs       # Configuration management
│   ├── monitor.rs      # Resource monitoring (CPU/GPU)
│   ├── worker.rs       # Parallel worker pool
│   └── gpu.rs          # CUDA kernel integration
└── target/release/     # Compiled output (after build)
```

## Troubleshooting

### "cargo: command not found"
- Run `.\setup_env.ps1` to add Cargo to PATH
- Or restart terminal after installing Rust
- Or manually add `%USERPROFILE%\.cargo\bin` to system PATH

### "Cannot find compiler 'cl.exe'"
- Install Visual Studio Build Tools with C++ workload
- Restart terminal after installation
- Verify with: `cl.exe` command

### "CUDA Toolkit not found"
- Install CUDA Toolkit from NVIDIA website
- Run `.\setup_env.ps1` to set CUDA paths
- Verify with: `nvcc --version`

### "GPU initialization failed"
- Ensure NVIDIA drivers are up to date
- Run `nvidia-smi` to check GPU status
- Verify CUDA installation: `nvcc --version`
- Check that GPU has compute capability 6.0+

### Compilation warnings about unused variables
- These are harmless warnings from code refactoring
- They don't affect functionality
- Can be ignored

## Output Files

- `mnemonics_log.txt` - Detailed log with timestamps
- `seeds_output.txt` - Simple list (mnemonic + char count)

Both files are created in the `rust/` directory.

## Comparison: Rust vs Python

| Feature | Python | Rust |
|---------|--------|------|
| Speed | ~1M it/s | ~1.5M it/s |
| Setup | Easy | Moderate |
| Compilation | None | Required |
| Memory | Higher | Lower |
| GPU Integration | PyTorch | Direct CUDA |
| Thread Safety | Manual | Compile-time |

## See Also

- Main README: `../README.md`
- Python version: `../python/README.md`
