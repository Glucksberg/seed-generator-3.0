# Instalação do CUDA Toolkit para Suporte GPU

## Status Atual

O projeto Rust está configurado para suporte GPU, mas requer o **CUDA Toolkit** instalado para funcionar completamente.

## Passos para Instalação

### 1. Verificar Compatibilidade

Sua GPU: **NVIDIA GeForce RTX 3080 Ti** ✅
- Suporta CUDA Compute Capability 8.6
- Driver atualizado: 581.57 ✅
- CUDA Version suportada pelo driver: 13.0 ✅

### 2. Baixar CUDA Toolkit

1. Acesse: https://developer.nvidia.com/cuda-downloads
2. Selecione:
   - **Operating System**: Windows
   - **Architecture**: x86_64
   - **Version**: Windows 10/11
   - **Installer Type**: exe (local)

3. Baixe o instalador (arquivo grande, ~3GB)

### 3. Instalar CUDA Toolkit

1. Execute o instalador baixado
2. Escolha **"Express Installation"** (instalação expressa)
3. Aguarde a instalação (pode levar 10-20 minutos)
4. **Reinicie o computador** após a instalação

### 4. Verificar Instalação

Abra PowerShell e execute:

```powershell
nvcc --version
```

Você deve ver algo como:
```
nvcc: NVIDIA (R) Cuda compiler driver
Copyright (c) 2005-2024 NVIDIA Corporation
Built on ...
Cuda compilation tools, release 13.x, V13.x.x
```

### 5. Recompilar o Projeto com Suporte GPU

Após instalar o CUDA Toolkit, recompile o projeto com suporte GPU:

```powershell
cd rust
cargo build --release --features gpu
```

### 6. Executar com GPU

Após recompilar, execute normalmente:

```powershell
.\target\release\gpuseed-rust.exe
```

Quando perguntado sobre GPU, escolha "Yes" e o programa usará a GPU para gerar entropia.

## Notas Importantes

- **Sem CUDA Toolkit**: O programa funciona normalmente, mas apenas em CPU
- **Com CUDA Toolkit**: O programa pode usar GPU para geração de entropia (muito mais rápido)
- O build.rs detecta automaticamente se o CUDA Toolkit está instalado
- Se não estiver instalado, o programa compila sem erros mas sem suporte GPU

## Troubleshooting

### Erro: "nvcc not found"
- Certifique-se de que o CUDA Toolkit foi instalado
- Reinicie o terminal/PowerShell após instalação
- Verifique se o PATH inclui: `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.x\bin`

### Erro: "CUDA cannot find"
- Verifique se os drivers NVIDIA estão atualizados
- Execute `nvidia-smi` para verificar se a GPU é detectada

### GPU não está sendo usada
- Certifique-se de compilar com `--features gpu`
- Verifique se escolheu "Yes" quando perguntado sobre GPU
- O programa faz fallback automático para CPU se GPU falhar



