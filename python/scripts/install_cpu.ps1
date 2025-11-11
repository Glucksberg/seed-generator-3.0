<# 
Purpose: Install project dependencies for CPU execution.
- Installs base requirements from requirements.txt
- Installs PyTorch (CPU-only) from the default PyPI index
- Validates that torch is importable and CUDA is not required
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "==> Preparing environment (CPU-only install)" -ForegroundColor Cyan

# Move to repo root (one level up from scripts/)
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location -Path $repoRoot

Write-Host "==> Python:" -ForegroundColor Yellow
python -c "import sys; print(sys.executable)"

Write-Host "==> Upgrading pip (optional)" -ForegroundColor Yellow
python -m pip install --upgrade pip

Write-Host "==> Installing base dependencies from requirements.txt" -ForegroundColor Yellow
python -m pip install -r requirements.txt

Write-Host "==> Installing PyTorch (CPU-only)" -ForegroundColor Yellow
python -m pip install torch

Write-Host "==> Validating installation" -ForegroundColor Yellow
python -c "import torch; print('torch:', torch.__version__); print('cuda available:', torch.cuda.is_available())"

Write-Host "==> Done (CPU-only environment ready)" -ForegroundColor Green




