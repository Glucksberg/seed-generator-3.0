# GPU Seed Generator - Versão Python

Versão original em Python do gerador de seeds BIP39.

## Características

- ✅ Suporte completo a GPU (CUDA/DirectML)
- ✅ Sistema de segurança (limite 80% CPU/GPU)
- ✅ Monitoramento em tempo real
- ✅ Throttling dinâmico
- ✅ Configuração interativa de GPU
- ✅ Instalação automática de dependências

## Instalação

```bash
# Instalar dependências
pip install -r requirements.txt

# (Opcional) Instalar PyTorch com CUDA para GPU NVIDIA
# Veja scripts/install_cuda.ps1

# (Opcional) Instalar PyTorch CPU-only
# Veja scripts/install_cpu.ps1
```

## Uso

```bash
# Uso básico
python gpuseed3.py --threshold 46 --count 5

# Com opções customizadas
python gpuseed3.py --threshold 44 --count 10 --batch-size 4096 --logfile resultados.txt

# Resetar configuração
python gpuseed3.py --reset-config
```

## Opções de Linha de Comando

- `--threshold <N>`: Limite de caracteres (padrão: 46)
- `--count <N>`: Número de mnemonics por contagem de caracteres (padrão: 5)
- `--logfile <arquivo>`: Arquivo de log detalhado (padrão: mnemonics_log.txt)
- `--batch-size <N>`: Tamanho do batch para GPU (padrão: 2048)
- `--reset-config`: Resetar configuração de GPU

## Requisitos

- Python 3.8+
- psutil (obrigatório)
- pynvml (opcional, para monitoramento NVIDIA)
- PyTorch (instalado automaticamente ou manualmente)
- mnemonic (biblioteca BIP39)
- colorama (cores no terminal)

## Performance

- CPU: ~18k-35k iterações/segundo
- GPU: ~30k-50k iterações/segundo (depende da GPU)

## Documentação Completa

Veja `../README.md` para documentação detalhada completa.

