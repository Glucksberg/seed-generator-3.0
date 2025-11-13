# PowerShell script to set up environment for Rust CUDA development
# Run this script before building: .\setup_env.ps1

Write-Host "Setting up Rust CUDA development environment..." -ForegroundColor Green

# Add Cargo to PATH if not already present
$cargoPath = "$env:USERPROFILE\.cargo\bin"
if (Test-Path $cargoPath) {
    if ($env:PATH -notlike "*$cargoPath*") {
        $env:PATH += ";$cargoPath"
        Write-Host "Added Cargo to PATH: $cargoPath" -ForegroundColor Yellow
    } else {
        Write-Host "Cargo already in PATH" -ForegroundColor Green
    }
} else {
    Write-Host "Warning: Cargo not found at $cargoPath" -ForegroundColor Red
}

# Set CUDA environment variables
$cudaPath = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0"
if (Test-Path $cudaPath) {
    $env:CUDA_PATH = $cudaPath
    $env:CUDA_LIBRARY_PATH = "$cudaPath\lib\x64"
    Write-Host "Set CUDA_PATH: $env:CUDA_PATH" -ForegroundColor Yellow
    Write-Host "Set CUDA_LIBRARY_PATH: $env:CUDA_LIBRARY_PATH" -ForegroundColor Yellow
} else {
    # Try to find CUDA in common locations
    $cudaVersions = @("v13.0", "v12.0", "v11.8")
    $found = $false
    foreach ($version in $cudaVersions) {
        $testPath = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\$version"
        if (Test-Path $testPath) {
            $env:CUDA_PATH = $testPath
            $env:CUDA_LIBRARY_PATH = "$testPath\lib\x64"
            Write-Host "Set CUDA_PATH: $env:CUDA_PATH" -ForegroundColor Yellow
            Write-Host "Set CUDA_LIBRARY_PATH: $env:CUDA_LIBRARY_PATH" -ForegroundColor Yellow
            $found = $true
            break
        }
    }
    if (-not $found) {
        Write-Host "Warning: CUDA not found in common locations" -ForegroundColor Red
    }
}

Write-Host "`nEnvironment setup complete!" -ForegroundColor Green
Write-Host "You can now run: cargo build --release --features gpu" -ForegroundColor Cyan



