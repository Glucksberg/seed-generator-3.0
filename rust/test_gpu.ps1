# Script de teste para verificar o modo GPU
# Este script executa o programa com parâmetros de teste

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Teste do Modo GPU - Seed Generator" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Verificar se o executável existe
$exePath = "target\release\gpuseed-rust.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "ERRO: Executável não encontrado em $exePath" -ForegroundColor Red
    Write-Host "Compile primeiro com: cargo build --release --features gpu" -ForegroundColor Yellow
    exit 1
}

Write-Host "Executável encontrado: $exePath" -ForegroundColor Green
Write-Host ""

# Teste 1: Verificar ajuda
Write-Host "Teste 1: Verificando ajuda do programa..." -ForegroundColor Yellow
& $exePath --help | Select-Object -First 5
Write-Host ""

# Teste 2: Resetar configuração e executar com parâmetros pequenos para teste rápido
Write-Host "Teste 2: Executando com reset de configuração..." -ForegroundColor Yellow
Write-Host "NOTA: O programa irá perguntar se você quer usar GPU." -ForegroundColor Cyan
Write-Host "      Escolha '1' para testar o modo GPU" -ForegroundColor Cyan
Write-Host "      Escolha '2' para testar o modo CPU" -ForegroundColor Cyan
Write-Host ""
Write-Host "Parâmetros do teste:" -ForegroundColor Yellow
Write-Host "  - Threshold: 46 caracteres" -ForegroundColor Gray
Write-Host "  - Count: 1 mnemonic (para teste rápido)" -ForegroundColor Gray
Write-Host "  - Batch size: 1024 (pequeno para teste)" -ForegroundColor Gray
Write-Host ""
Write-Host "Pressione qualquer tecla para iniciar o teste..." -ForegroundColor Green
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

Write-Host ""
Write-Host "Iniciando teste..." -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Executar com parâmetros de teste (threshold baixo e count pequeno para teste rápido)
& $exePath --threshold 46 --count 1 --batch-size 1024 --output test_output.txt --reset-config

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Teste concluído!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Verifique:" -ForegroundColor Yellow
Write-Host "  1. Se o programa detectou e inicializou a GPU" -ForegroundColor Gray
Write-Host "  2. Se apareceu 'GPU initialized successfully (CUDA)'" -ForegroundColor Gray
Write-Host "  3. Se o status mostra uso de GPU > 0%" -ForegroundColor Gray
Write-Host "  4. Se o arquivo test_output.txt foi criado" -ForegroundColor Gray
Write-Host ""

if (Test-Path "test_output.txt") {
    Write-Host "Arquivo de saída criado: test_output.txt" -ForegroundColor Green
    $content = Get-Content "test_output.txt" -ErrorAction SilentlyContinue
    if ($content) {
        Write-Host "Conteúdo:" -ForegroundColor Yellow
        $content | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
    }
}



