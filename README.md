# GPU Seed Generator 3.0

High-performance BIP39 mnemonic seed generator with GPU acceleration.

## Overview

Generates BIP39 mnemonic seeds and searches for those with minimal character counts (spaces removed). Uses GPU acceleration to achieve billions of iterations per hour.

**Key Features:**
- GPU acceleration via NVIDIA CUDA
- Automatic resource throttling (80% CPU/GPU safety limit)
- Searches for seeds with < 46 characters
- No limit for rare seeds ≤ 42 characters
- Limit of 5 for seeds with 43-45 characters
- Real-time progress monitoring

## Quick Start

### Rust Version (Recommended - Fastest)
```bash
cd rust
run.bat
```

### Python Version
```bash
cd python
run.bat
```

## Implementations

### Rust (Best Performance)
- **Speed**: 1.5-2 billion iterations/hour
- **Technology**: Direct CUDA kernel integration with cuRAND
- **Location**: `rust/`
- **Setup**: See `rust/README.md`

### Python (Easier Setup)
- **Speed**: 1-1.5 billion iterations/hour
- **Technology**: PyTorch GPU acceleration
- **Location**: `python/`
- **Setup**: See `python/README.md`

## Requirements

**Hardware:**
- NVIDIA GPU with CUDA support

**Software:**
- CUDA Toolkit 11.8 or 12.x
- For Rust: Visual Studio Build Tools + Rust toolchain
- For Python: Python 3.8+ + PyTorch with CUDA

## Output Files

Results saved to:
- `mnemonics_log.txt` - Detailed log with timestamps and iterations
- `seeds_output.txt` - Simple list format (mnemonic + char count)

## Seed Collection Rules

- **45, 44, 43 characters**: Maximum 5 seeds per character count
- **42 characters or less**: Unlimited (collects all unique seeds found)

## Performance

Real-world performance with NVIDIA GPU:
- **Rust**: ~1,500,000 iterations/second
- **Python**: ~1,000,000 iterations/second
- **2 hours runtime**: ~3 billion seeds tested

## Documentation

- **Rust Setup & Usage**: `rust/README.md`
- **Python Setup & Usage**: `python/README.md`

## How to Choose

- **Use Rust** if you want maximum speed and don't mind compilation
- **Use Python** if you want easier setup or need to modify code frequently

Both versions produce identical results and share configuration.

## License

Educational and research purposes only.
