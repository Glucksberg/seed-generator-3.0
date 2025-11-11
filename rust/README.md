# GPU Seed Generator - Versão Rust

Versão de alta performance escrita em Rust.

## Características

- ✅ **Alta Performance**: 10-50x mais rápido que Python
- ✅ **Thread-Safe**: Garantido pelo compilador Rust
- ✅ **Sistema de Segurança**: Limite de 80% CPU/GPU
- ✅ **Processamento Paralelo**: Usa todos os cores eficientemente
- ⏳ **GPU**: Preparado para CUDA (a ser implementado)

## Requisitos

- Rust 1.70 ou superior
- Cargo (geralmente vem com Rust)

## Instalação

```bash
# Navegar até a pasta rust
cd rust

# Compilar o projeto
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

- [ ] Implementar suporte completo a CUDA
- [ ] Adicionar monitoramento de GPU via NVML
- [ ] Otimizar com SIMD para processamento vetorizado
- [ ] Adicionar suporte a DirectML (AMD/Intel)
- [ ] Implementar processamento em batch na GPU

## Documentação Técnica

Veja `../REVISAO_FINAL.md` para revisão técnica completa do código.

