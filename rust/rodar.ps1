# Script para executar o programa facilmente
# Uso: .\rodar.ps1 [threshold] [count] [batch-size]

param(
    [int]$threshold = 46,
    [int]$count = 10,
    [int]$batchSize = 32768
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GPU Seed Generator - Execução" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Verificar se o ambiente está configurado
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Configurando ambiente..." -ForegroundColor Yellow
    .\setup_env.ps1
    Write-Host ""
}

# Verificar se o executável existe
$exePath = "target\release\gpuseed-rust.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "ERRO: Executável não encontrado!" -ForegroundColor Red
    Write-Host "Compile primeiro com: cargo build --release --features gpu" -ForegroundColor Yellow
    exit 1
}

Write-Host "Parâmetros:" -ForegroundColor Green
Write-Host "  Threshold: $threshold caracteres" -ForegroundColor Gray
Write-Host "  Count: $count mnemonics" -ForegroundColor Gray
Write-Host "  Batch Size: $batchSize" -ForegroundColor Gray
Write-Host ""

Write-Host "Iniciando..." -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Executar o programa
& $exePath --threshold $threshold --count $count --batch-size $batchSize

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Execução concluída!" -ForegroundColor Green



