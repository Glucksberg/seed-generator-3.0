# Seed Generator 3.0

Gerador de mnemonics BIP39 otimizado com processamento paralelo usando GPU (CUDA) e multiprocessamento.

## 📋 Descrição

Este script gera mnemonics BIP39 aleatórios e busca por aqueles que atendem a um critério de contagem de caracteres. Utiliza processamento paralelo com múltiplos processos e aproveitamento de GPU quando disponível para máxima performance.

## 🚀 Características

- **Processamento GPU**: Utiliza CUDA quando disponível para geração rápida de entropia
- **Multiprocessamento**: Múltiplos workers paralelos para geração de mnemonics
- **Fila otimizada**: Sistema de fila com ajuste dinâmico de batch size
- **Interface colorida**: Saída colorida no terminal usando colorama
- **Logging**: Salva resultados em arquivo de log
- **Interrupção segura**: Pressione 'q' e Enter para parar o script graciosamente

## 📦 Requisitos

- Python 3.7 ou superior
- GPU NVIDIA com suporte CUDA (opcional, mas recomendado para melhor performance)
- CUDA Toolkit instalado (se usar GPU)

## 🔧 Instalação

1. Clone o repositório:
```bash
git clone <url-do-repositorio>
cd seed-generator-3.0
```

2. Instale as dependências:
```bash
pip install -r requirements.txt
```

### Instalação do PyTorch com suporte CUDA (Opcional)

Se você tem uma GPU NVIDIA e quer aproveitar o processamento GPU, instale o PyTorch com suporte CUDA:

**Para CUDA 11.8:**
```bash
pip install torch --index-url https://download.pytorch.org/whl/cu118
```

**Para CUDA 12.1:**
```bash
pip install torch --index-url https://download.pytorch.org/whl/cu121
```

**Para CPU apenas:**
```bash
pip install torch
```

Verifique a versão do seu CUDA antes de instalar:
```bash
nvcc --version
```

## 📖 Uso

### Uso Básico

Execute o script com os parâmetros padrão:
```bash
python gpuseed3.py
```

### Parâmetros Disponíveis

- `--threshold`: Limite de caracteres para filtrar mnemonics (padrão: 45)
- `--count`: Número de mnemonics a encontrar por contagem de caracteres (padrão: 5)
- `--logfile`: Nome do arquivo de log (padrão: mnemonics_log.txt)
- `--batch-size`: Tamanho do batch para processamento GPU (padrão: 2048)

### Exemplos

Buscar mnemonics com 40 caracteres ou menos:
```bash
python gpuseed3.py --threshold 40
```

Encontrar 10 mnemonics por contagem de caracteres:
```bash
python gpuseed3.py --count 10
```

Usar batch size maior para melhor performance:
```bash
python gpuseed3.py --batch-size 4096
```

Combinar parâmetros:
```bash
python gpuseed3.py --threshold 42 --count 3 --logfile resultados.txt --batch-size 4096
```

### Parar o Script

Para interromper o script de forma segura, pressione `q` e depois `Enter`.

## 📊 Saída

O script exibe no console:
- Mnemonics encontrados que atendem ao critério
- Contagem total de caracteres
- Número de iterações processadas
- Tempo decorrido
- Estatísticas de processamento (mnemonics/s)

Os resultados também são salvos no arquivo de log especificado.

## ⚙️ Configurações Avançadas

Você pode ajustar as constantes no início do arquivo `gpuseed3.py`:

- `NUM_STREAMS`: Número de streams CUDA para processamento paralelo (padrão: 4)
- `QUEUE_MAX_SIZE`: Tamanho máximo da fila (padrão: 50000)
- `WORKER_DELAY`: Delay base nos workers (padrão: 0.01)
- `QUEUE_WARNING_THRESHOLD`: Limite para desacelerar workers (padrão: 0.9)
- `BATCH_SIZE_MIN`: Tamanho mínimo do batch (padrão: 512)

## 🐛 Troubleshooting

### PyTorch não detecta GPU

1. Verifique se o CUDA está instalado:
```bash
nvcc --version
```

2. Verifique se o PyTorch foi instalado com suporte CUDA:
```python
python -c "import torch; print(torch.cuda.is_available())"
```

3. Se retornar `False`, reinstale o PyTorch com suporte CUDA apropriado.

### Erro ao importar mnemonic

Certifique-se de que a biblioteca foi instalada corretamente:
```bash
pip install mnemonic
```

### Performance baixa

- Aumente o `--batch-size` se tiver GPU disponível
- Verifique se está usando GPU (o script informa no início)
- Ajuste o número de processos conforme seu hardware

## 📝 Notas

- O script usa multiprocessamento, então aproveita todos os cores disponíveis
- A fila ajusta dinamicamente o batch size para evitar sobrecarga
- Os resultados são salvos incrementalmente no arquivo de log
- Mnemonics duplicados são automaticamente ignorados

## 📄 Licença

[Especifique a licença do seu projeto aqui]

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

