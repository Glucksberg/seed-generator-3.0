<# 
Purpose: Install project dependencies with NVIDIA CUDA acceleration.
- Installs base requirements from requirements.txt
- Uninstalls any existing CPU torch
- Tries to install PyTorch with CUDA 12.1; falls back to CUDA 11.8 if needed
- Validates CUDA availability and prints GPU name
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "==> Preparing environment (CUDA install)" -ForegroundColor Cyan

# Move to repo root (one level up from scripts/)
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location -Path $repoRoot

Write-Host "==> Python:" -ForegroundColor Yellow
python -c "import sys; print(sys.executable)"

Write-Host "==> NVIDIA driver / GPU (optional)" -ForegroundColor Yellow
try { nvidia-smi } catch { Write-Host "nvidia-smi not found (ensure NVIDIA driver is installed)" -ForegroundColor DarkYellow }

Write-Host "==> Upgrading pip (optional)" -ForegroundColor Yellow
python -m pip install --upgrade pip

Write-Host "==> Installing base dependencies from requirements.txt" -ForegroundColor Yellow
python -m pip install -r requirements.txt

Write-Host "==> Removing any existing torch CPU packages" -ForegroundColor Yellow
python -m pip uninstall -y torch torchvision torchaudio

Write-Host "==> Installing PyTorch (CUDA 12.1)..." -ForegroundColor Yellow
python -m pip install --upgrade --index-url https://download.pytorch.org/whl/cu121 torch
if ($LASTEXITCODE -ne 0) {
    Write-Host "CUDA 12.1 build not available for this Python/OS. Falling back to CUDA 11.8..." -ForegroundColor DarkYellow
    python -m pip install --upgrade --index-url https://download.pytorch.org/whl/cu118 torch
}

Write-Host "==> Validating CUDA" -ForegroundColor Yellow
python -c "import torch; print('torch:', torch.__version__); print('cuda available:', torch.cuda.is_available()); print('torch.version.cuda:', torch.version.cuda); print(torch.cuda.get_device_name(0) if torch.cuda.is_available() else 'no cuda device')"

Write-Host "==> Done (CUDA environment ready)" -ForegroundColor Green



