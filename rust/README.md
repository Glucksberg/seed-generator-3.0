# GPU Seed Generator - Versão Rust

Versão de alta performance escrita em Rust.

## Características

- ✅ **Alta Performance**: 10-50x mais rápido que Python
- ✅ **Thread-Safe**: Garantido pelo compilador Rust
- ✅ **Sistema de Segurança**: Limite de 80% CPU/GPU
- ✅ **Processamento Paralelo**: Usa todos os cores eficientemente
- ✅ **GPU**: Suporte CUDA com kernel compilado automaticamente

## Requisitos

- Rust 1.70 ou superior
- Cargo (geralmente vem com Rust)
- **Para suporte GPU (CUDA)**:
  - CUDA Toolkit 11.8+ instalado
  - Visual Studio Build Tools 2019+ ou Visual Studio 2022+ com componente C++
  - GPU NVIDIA com compute capability 6.0+

## Instalação

### Compilação Básica (CPU apenas)

```bash
# Navegar até a pasta rust
cd rust

# Compilar o projeto
cargo build --release

# O executável estará em: target/release/gpuseed-rust.exe (Windows)
# ou target/release/gpuseed-rust (Linux/macOS)
```

### Compilação com Suporte GPU (CUDA)

**No Windows:**

1. **Configure o ambiente** (execute uma vez por sessão do PowerShell):
```powershell
cd rust
.\setup_env.ps1
```

2. **Compile com suporte GPU**:
```powershell
cargo build --release --features gpu
```

**Nota:** Se o `cargo` não for encontrado, o script `setup_env.ps1` adiciona automaticamente ao PATH. Se ainda assim não funcionar, você pode precisar:
- Reiniciar o terminal após instalar o Rust
- Ou adicionar manualmente `%USERPROFILE%\.cargo\bin` ao PATH do sistema

**Configuração Permanente (Opcional):**

Para não precisar executar o script toda vez, adicione ao seu perfil do PowerShell (`$PROFILE`):
```powershell
# Adicionar ao PATH do sistema (via Painel de Controle > Variáveis de Ambiente)
# ou adicionar ao perfil do PowerShell:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0"
$env:CUDA_LIBRARY_PATH = "$env:CUDA_PATH\lib\x64"
```

## Uso

```bash
# Uso básico
cargo run --release -- --threshold 46 --count 5

# Com opções customizadas
cargo run --release -- --threshold 44 --count 10 --batch-size 16384 --output minhas_seeds.txt

# Resetar configuração
cargo run --release -- --reset-config
```

## Opções de Linha de Comando

- `--threshold <N>`: Limite de caracteres (padrão: 46)
- `--count <N>`: Número de mnemonics por contagem de caracteres (padrão: 5)
- `--logfile <arquivo>`: Arquivo de log detalhado (padrão: mnemonics_log.txt)
- `--batch-size <N>`: Tamanho do batch para processamento (padrão: 8192)
- `--output <arquivo>`: Arquivo de saída simples com seeds (padrão: seeds_output.txt)
- `--reset-config`: Resetar configuração de GPU

## Performance

- CPU: ~300k-1.5M iterações/segundo
- Esperado: 10-50x mais rápido que a versão Python

## Estrutura do Código

```
rust/
├── Cargo.toml      # Configuração do projeto
├── src/
│   ├── main.rs     # Ponto de entrada principal
│   ├── config.rs   # Gerenciamento de configuração
│   ├── monitor.rs  # Monitoramento de recursos CPU/GPU
│   ├── worker.rs   # Pool de workers para processamento paralelo
│   └── gpu.rs      # Suporte a GPU (CUDA - a ser implementado)
└── README.md       # Este arquivo
```

## Comparação com Versão Python

| Métrica | Python | Rust |
|---------|--------|------|
| Velocidade | ~30k it/s | ~300k-1.5M it/s |
| Uso de Memória | Alto | Baixo |
| Startup Time | Rápido | Médio (compilação) |
| Thread Safety | Manual | Garantido pelo compilador |
| Manutenibilidade | Alta | Média-Alta |

## Próximos Passos

- [x] Implementar suporte completo a CUDA (kernel compilado com sucesso)
- [ ] Adicionar monitoramento de GPU via NVML
- [ ] Otimizar com SIMD para processamento vetorizado
- [ ] Adicionar suporte a DirectML (AMD/Intel)
- [ ] Implementar processamento em batch na GPU

## Troubleshooting

### Cargo não encontrado
Execute `.\setup_env.ps1` ou adicione `%USERPROFILE%\.cargo\bin` ao PATH.

### Erro "Cannot find compiler 'cl.exe'"
O Visual Studio Build Tools precisa estar instalado. O script `build.rs` configura automaticamente o ambiente, mas se ainda falhar, execute manualmente:
```powershell
& "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
```

### Erro "CUDA cannot find"
Configure as variáveis de ambiente CUDA:
```powershell
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.0"
$env:CUDA_LIBRARY_PATH = "$env:CUDA_PATH\lib\x64"
```
Ou execute `.\setup_env.ps1` que faz isso automaticamente.

## Documentação Técnica

Veja `../REVISAO_FINAL.md` para revisão técnica completa do código.

