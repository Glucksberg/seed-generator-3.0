# GPU Seed Generator - Rust Version

Uma versão de alta performance do gerador de seeds BIP39 escrita em Rust.

## Características

- **Alta Performance**: 10-50x mais rápido que a versão Python
- **Processamento Paralelo**: Usa todos os cores da CPU eficientemente
- **Monitoramento de Recursos**: Sistema de segurança que limita uso de CPU/GPU a 80%
- **Suporte a GPU**: Preparado para CUDA (a ser implementado)
- **Thread-Safe**: Usa Arc/Mutex para compartilhamento seguro de dados
- **Cross-Platform**: Funciona em Windows, Linux e macOS

## Requisitos

- Rust 1.70 ou superior
- Cargo (geralmente vem com Rust)

## Instalação

```bash
# Clone ou navegue até o diretório
cd seed-generator-3.0

# Compile o projeto
cargo build --release

# O executável estará em: target/release/gpuseed-rust.exe (Windows)
# ou target/release/gpuseed-rust (Linux/macOS)
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

## Comparação com Versão Python

| Métrica | Python | Rust |
|---------|--------|------|
| Velocidade | ~30k it/s | ~300k-1.5M it/s |
| Uso de Memória | Alto | Baixo |
| Startup Time | Rápido | Médio (compilação) |
| Manutenibilidade | Alta | Média-Alta |

## Estrutura do Projeto

```
src/
├── main.rs      # Ponto de entrada principal
├── config.rs    # Gerenciamento de configuração
├── monitor.rs   # Monitoramento de recursos CPU/GPU
├── worker.rs    # Pool de workers para processamento paralelo
└── gpu.rs       # Suporte a GPU (CUDA - a ser implementado)
```

## Próximos Passos

- [ ] Implementar suporte completo a CUDA
- [ ] Adicionar monitoramento de GPU via NVML
- [ ] Otimizar com SIMD para processamento vetorizado
- [ ] Adicionar suporte a DirectML (AMD/Intel)
- [ ] Implementar processamento em batch na GPU

## Notas

Esta é uma versão inicial. O suporte completo a GPU será adicionado em versões futuras.

