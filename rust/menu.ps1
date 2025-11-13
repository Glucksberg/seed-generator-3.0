# Menu interativo para executar o programa
# Uso: .\menu.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GPU Seed Generator - Menu" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Escolha uma opção:" -ForegroundColor Yellow
Write-Host "1. Busca Rápida (threshold 44, 5 resultados)" -ForegroundColor Gray
Write-Host "2. Busca Padrão (threshold 46, 10 resultados)" -ForegroundColor Gray
Write-Host "3. Busca Intensa (threshold 48, 20 resultados)" -ForegroundColor Gray
Write-Host "4. Personalizado (você escolhe os parâmetros)" -ForegroundColor Gray
Write-Host ""
$choice = Read-Host "Digite o número da opção"

# Configurar ambiente se necessário
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Configurando ambiente..." -ForegroundColor Yellow
    .\setup_env.ps1
    Write-Host ""
}

$exePath = "target\release\gpuseed-rust.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "ERRO: Executável não encontrado!" -ForegroundColor Red
    Write-Host "Compile primeiro com: cargo build --release --features gpu" -ForegroundColor Yellow
    exit 1
}

switch ($choice) {
    "1" {
        Write-Host "Executando busca rápida..." -ForegroundColor Green
        & $exePath --threshold 44 --count 5 --batch-size 32768
    }
    "2" {
        Write-Host "Executando busca padrão..." -ForegroundColor Green
        & $exePath --threshold 46 --count 10 --batch-size 32768
    }
    "3" {
        Write-Host "Executando busca intensa..." -ForegroundColor Green
        & $exePath --threshold 48 --count 20 --batch-size 65536
    }
    "4" {
        $th = Read-Host "Threshold (padrão: 46)"
        $cnt = Read-Host "Count (padrão: 10)"
        $bs = Read-Host "Batch Size (padrão: 32768)"
        
        if ([string]::IsNullOrWhiteSpace($th)) { $th = 46 }
        if ([string]::IsNullOrWhiteSpace($cnt)) { $cnt = 10 }
        if ([string]::IsNullOrWhiteSpace($bs)) { $bs = 32768 }
        
        Write-Host "Executando com parâmetros personalizados..." -ForegroundColor Green
        & $exePath --threshold $th --count $cnt --batch-size $bs
    }
    default {
        Write-Host "Opção inválida!" -ForegroundColor Red
        exit 1
    }
}



