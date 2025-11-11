# Seed Generator 3.0

Gerador de mnemonics BIP39 otimizado com processamento paralelo usando GPU (CUDA) e multiprocessamento.

## 📋 Descrição

Este script gera mnemonics BIP39 aleatórios e busca por aqueles que atendem a um critério de contagem de caracteres. Utiliza processamento paralelo com múltiplos processos e aproveitamento de GPU quando disponível para máxima performance.

## 🚀 Características

- **Processamento GPU**: Utiliza CUDA quando disponível para geração rápida de entropia
- **Multiprocessamento**: Múltiplos workers paralelos para geração de mnemonics
- **Fila otimizada**: Sistema de fila com ajuste dinâmico de batch size
- **Sistema de Segurança**: Limita automaticamente o uso de CPU e GPU a 80% para proteger o hardware
- **Monitoramento em Tempo Real**: Exibe uso de CPU e GPU durante a execução
- **Throttling Inteligente**: Reduz automaticamente a carga quando os limites são atingidos
- **Interface colorida**: Saída colorida no terminal usando colorama
- **Logging**: Salva resultados em arquivo de log
- **Interrupção segura**: Pressione 'q' e Enter para parar o script graciosamente

## 📦 Requisitos

- Python 3.8 ou superior
- **Dependências obrigatórias** (instaladas automaticamente):
  - `mnemonic>=0.20`: Geração de mnemonics BIP39
  - `colorama>=0.4.6`: Cores no terminal
  - `psutil>=5.9.0`: Monitoramento de CPU e recursos do sistema
- **Dependências opcionais**:
  - `pynvml>=11.5.0`: Monitoramento avançado de GPU NVIDIA (recomendado para GPUs NVIDIA)
- **Aceleração por GPU** (opcional):
  - NVIDIA (CUDA): Driver NVIDIA e PyTorch com CUDA
  - AMD/Intel (Windows): `torch-directml`

## 🔧 Instalação

1. Clone o repositório:
```bash
git clone <url-do-repositorio>
cd seed-generator-3.0
```

2. Instale as dependências base:
```bash
pip install -r requirements.txt
```

3. (Opcional) Para monitoramento avançado de GPU NVIDIA:
```bash
pip install pynvml
```

**Nota sobre o aviso do pip**: Se você receber um aviso sobre scripts do pip não estarem no PATH, isso não afeta o funcionamento do programa. É apenas informativo. Se quiser corrigir (opcional), adicione `C:\Users\[SeuUsuario]\AppData\Roaming\Python\Python313\Scripts` ao PATH do Windows.

### Instalação rápida via scripts (Windows/PowerShell)

Execute um dos scripts abaixo a partir do diretório do projeto:

- CPU (sem GPU):
```powershell
PowerShell -ExecutionPolicy Bypass -File .\scripts\install_cpu.ps1
```
- NVIDIA CUDA:
```powershell
PowerShell -ExecutionPolicy Bypass -File .\scripts\install_cuda.ps1
```

### Instalação do PyTorch para GPU (opcional)

Escolha UMA opção conforme seu hardware:

#### Opção A) NVIDIA (CUDA)

Verifique o driver:
```bash
nvidia-smi
```

Instale o PyTorch com CUDA (selecione a versão):
```bash
pip uninstall -y torch torchvision torchaudio
pip install --upgrade --index-url https://download.pytorch.org/whl/cu121 torch  # CUDA 12.1
# ou
pip install --upgrade --index-url https://download.pytorch.org/whl/cu118 torch  # CUDA 11.8
```

#### Opção B) AMD/Intel (Windows - DirectML)
```bash
pip uninstall -y torch torchvision torchaudio
pip install torch-directml
```

### Validação da instalação
```bash
python -c "import torch; print('torch:', torch.__version__); print('cuda available:', torch.cuda.is_available()); import importlib; print('directml available:', importlib.util.find_spec('torch_directml') is not None)"
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

Usar batch size maior para melhor performance (GPU):
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
- **Uso de CPU e GPU em tempo real**
- **Status de throttling** quando o sistema de segurança está ativo

Exemplo de saída:
```
Processed: 1,234,567 (12,345/s) | CPU: 75.2% | GPU: 78.5%
```

Quando o throttling está ativo:
```
Processed: 1,234,567 (12,345/s) | CPU: 85.3% | GPU: 82.1% [THROTTLE: CPU=0.94, GPU=0.97]
```

Os resultados também são salvos no arquivo de log especificado.

## ⚙️ Configurações Avançadas

Você pode ajustar as constantes no início do arquivo `gpuseed3.py`:

- `NUM_STREAMS`: Número de streams CUDA para processamento paralelo (padrão: 4)
- `QUEUE_MAX_SIZE`: Tamanho máximo da fila (padrão: 50000)
- `WORKER_DELAY`: Delay base nos workers (padrão: 0.01)
- `QUEUE_WARNING_THRESHOLD`: Limite para desacelerar workers (padrão: 0.9)
- `BATCH_SIZE_MIN`: Tamanho mínimo do batch (padrão: 512)
- `MAX_USAGE_PERCENT`: Limite máximo de uso de CPU/GPU para o sistema de segurança (padrão: 0.80 = 80%)
- `MONITOR_INTERVAL`: Intervalo de verificação de recursos em segundos (padrão: 0.5)
- `THROTTLE_DELAY_BASE`: Delay base quando throttling está ativo (padrão: 0.05)

### Sistema de Segurança

O programa inclui um sistema de segurança que monitora continuamente o uso de CPU e GPU. Quando o uso excede 80% (configurável via `MAX_USAGE_PERCENT`), o sistema automaticamente:

- Reduz o tamanho dos batches processados
- Diminui o número de streams CUDA (se aplicável)
- Adiciona delays entre operações
- Ajusta dinamicamente a carga para manter o uso abaixo do limite

Isso protege seu hardware contra sobrecarga e permite que o sistema continue responsivo durante a execução.

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
- O sistema de segurança pode estar limitando a performance se o uso estiver próximo de 80%

### Monitoramento de recursos não funciona

- **CPU**: Certifique-se de que `psutil` está instalado (`pip install psutil`)
- **GPU NVIDIA**: Para monitoramento preciso, instale `pynvml` (`pip install pynvml`)
- Sem `pynvml`, o sistema usa estimativas baseadas em memória CUDA (menos preciso)
- GPUs DirectML (AMD/Intel) não têm monitoramento direto disponível

## 📝 Notas

- O script usa multiprocessamento, então aproveita todos os cores disponíveis
- A fila ajusta dinamicamente o batch size para evitar sobrecarga
- O sistema de segurança monitora e limita o uso de recursos automaticamente
- Os resultados são salvos incrementalmente no arquivo de log
- Mnemonics duplicados são automaticamente ignorados
- O monitoramento de recursos roda em um processo separado para não impactar a performance
- O sistema é tolerante a falhas: se o monitoramento falhar, o throttling é desabilitado para não interromper o trabalho

## 📄 Licença

[Especifique a licença do seu projeto aqui]

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

