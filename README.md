# GPU Seed Generator 3.0

Gerador de seeds BIP39 de alta performance com suporte a GPU (CUDA/DirectML) e sistema de segurança para limitar uso de recursos.

## 📁 Estrutura do Projeto

```
seed-generator-3.0/
├── python/              # Versão Python (original)
│   ├── gpuseed3.py     # Script principal Python
│   ├── requirements.txt # Dependências Python
│   ├── scripts/        # Scripts de instalação
│   └── README.md       # Documentação da versão Python
├── rust/               # Versão Rust (alta performance)
│   ├── Cargo.toml     # Configuração do projeto Rust
│   ├── src/           # Código fonte Rust
│   ├── README.md      # Documentação da versão Rust
│   └── README_RUST.md # Documentação técnica adicional
├── README.md          # Este arquivo (documentação geral)
├── REVISAO_FINAL.md   # Revisão técnica completa
└── gpuseed_config.json # Configuração compartilhada (gerado automaticamente)
```

## 🚀 Versões Disponíveis

### Versão Python (`python/`)
- ✅ Funcional e testada
- ✅ Suporte completo a GPU (CUDA/DirectML)
- ✅ Sistema de segurança (limite 80%)
- ✅ Fácil de usar e modificar
- 📊 Performance: ~30k iterações/segundo

**Uso:**
```bash
cd python
pip install -r requirements.txt
python gpuseed3.py --threshold 46 --count 5
```

**Documentação completa:** Veja `python/README.md`

### Versão Rust (`rust/`)
- ✅ Alta performance (10-50x mais rápido)
- ✅ Thread-safe garantido pelo compilador
- ✅ Sistema de segurança (limite 80%)
- ⏳ Suporte GPU (preparado, a ser implementado)
- 📊 Performance: ~300k-1.5M iterações/segundo

**Uso:**
```bash
cd rust
cargo build --release
cargo run --release -- --threshold 46 --count 5
```

**Documentação completa:** Veja `rust/README.md`

## 📋 Arquivos Compartilhados (Raiz)

- `gpuseed_config.json` - Configuração de GPU (compartilhada entre versões)
- `*.txt` - Arquivos de log/output (gerados durante execução)
- `README.md` - Este arquivo
- `REVISAO_FINAL.md` - Revisão técnica completa

## 🎯 Qual Versão Usar?

- **Python**: Se você quer facilidade de uso, modificação rápida, ou já tem Python instalado
- **Rust**: Se você precisa de máxima performance e está disposto a compilar

Ambas as versões são funcionalmente equivalentes e compartilham a mesma configuração (`gpuseed_config.json`).

## 📖 Documentação Detalhada

- **Versão Python**: Veja `python/README.md`
- **Versão Rust**: Veja `rust/README.md`
- **Revisão Técnica**: Veja `REVISAO_FINAL.md`

## 🔧 Requisitos

### Python
- Python 3.8+
- pip
- (Opcional) PyTorch com CUDA para GPU

### Rust
- Rust 1.70+
- Cargo (vem com Rust)

## 📝 Notas

- Ambas as versões geram o mesmo arquivo de configuração (`gpuseed_config.json`)
- Os arquivos de output podem ser compartilhados entre versões
- A versão Rust é significativamente mais rápida mas requer compilação
- Cada versão tem sua própria documentação na respectiva pasta
