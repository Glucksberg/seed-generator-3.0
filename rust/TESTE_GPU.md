# Guia Rápido de Teste - Modo GPU

## Teste Rápido (Recomendado)

Execute o script de teste automatizado:
```powershell
cd rust
.\test_gpu.ps1
```

## Teste Manual

### 1. Teste Básico com GPU

```powershell
cd rust
.\setup_env.ps1  # Configure o ambiente se ainda não fez
.\target\release\gpuseed-rust.exe --threshold 46 --count 1 --batch-size 1024 --reset-config
```

Quando o programa perguntar:
- **Escolha "1"** para usar GPU
- **Escolha "1"** novamente para NVIDIA

### 2. O que observar durante o teste:

✅ **Sucesso do GPU:**
- Mensagem: "GPU initialized successfully (CUDA)"
- Status mostra: "GPU: X.X%" (onde X > 0)
- Performance muito mais alta que CPU

❌ **Se GPU falhar:**
- Mensagem: "GPU initialization failed: ..."
- Programa continua em modo CPU
- Status mostra: "GPU: 0.0%"

### 3. Teste de Performance Comparativo

**Modo CPU:**
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5 --batch-size 8192
# Escolha "2" quando perguntar sobre GPU
```

**Modo GPU:**
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5 --batch-size 8192
# Escolha "1" quando perguntar sobre GPU, depois "1" para NVIDIA
```

Compare as velocidades (iterações/segundo) entre os dois modos.

### 4. Verificar se o Kernel CUDA foi compilado

O kernel CUDA deve ter sido compilado durante o build. Verifique:
```powershell
Test-Path "target\release\build\gpuseed-rust-*\out\kernel.ptx"
```

Se existir, o kernel foi compilado com sucesso!

### 5. Troubleshooting

**Se aparecer erro sobre CUDA:**
- Verifique se a GPU NVIDIA está conectada e funcionando
- Execute: `nvidia-smi` para verificar se o driver está funcionando
- Verifique se CUDA_PATH está configurado: `$env:CUDA_PATH`

**Se o programa não encontrar o cargo:**
- Execute: `.\setup_env.ps1` novamente

**Se aparecer erro sobre cl.exe:**
- O build.rs deve ter configurado automaticamente
- Se persistir, verifique se Visual Studio Build Tools está instalado

## Resultados Esperados

Com GPU funcionando corretamente:
- ✅ Inicialização: "GPU initialized successfully (CUDA)"
- ✅ Status: GPU usage > 0%
- ✅ Performance: Muito mais rápido que CPU (pode variar)
- ✅ Sem erros relacionados a CUDA

Com GPU não disponível ou desabilitado:
- ✅ Programa continua normalmente em modo CPU
- ✅ Status: GPU: 0.0%
- ✅ Performance: Normal para CPU



