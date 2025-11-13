# Guia de Uso Real - GPU Seed Generator

## 🚀 Início Rápido

### 1. Configure o ambiente (só precisa fazer uma vez por sessão)
```powershell
cd rust
.\setup_env.ps1
```

### 2. Execute o programa
```powershell
.\target\release\gpuseed-rust.exe
```

Na primeira execução, o programa vai perguntar se você quer usar GPU. Escolha:
- **1** para usar GPU (muito mais rápido)
- **2** para usar apenas CPU

## 📋 Parâmetros Disponíveis

### Uso Básico
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5
```

### Parâmetros Explicados:

- `--threshold <N>`: Limite de caracteres no mnemonic (padrão: 46)
  - Valores comuns: 44-50
  - Quanto maior, mais raro (mais difícil de encontrar)
  
- `--count <N>`: Quantos mnemonics encontrar por threshold (padrão: 5)
  - Quantos resultados você quer antes de parar
  
- `--batch-size <N>`: Tamanho do lote para processamento (padrão: 8192)
  - Valores maiores = mais rápido, mas mais uso de memória
  - Recomendado: 16384-65536 para GPU, 8192-16384 para CPU
  
- `--output <arquivo>`: Arquivo onde salvar os seeds encontrados (padrão: seeds_output.txt)
  
- `--logfile <arquivo>`: Arquivo de log detalhado (padrão: mnemonics_log.txt)
  
- `--reset-config`: Resetar configuração de GPU (volta a perguntar sobre GPU)

## 🎯 Exemplos de Uso Real

### Exemplo 1: Busca Padrão (Recomendado)
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 10 --batch-size 16384
```
- Procura 10 mnemonics com 46+ caracteres
- Usa batch size otimizado para GPU
- Salva em `seeds_output.txt`

### Exemplo 2: Busca Rápida (Poucos Resultados)
```powershell
.\target\release\gpuseed-rust.exe --threshold 44 --count 3 --batch-size 32768
```
- Threshold menor = encontra mais rápido
- Apenas 3 resultados
- Batch size maior para máxima velocidade

### Exemplo 3: Busca Intensa (Muitos Resultados)
```powershell
.\target\release\gpuseed-rust.exe --threshold 48 --count 50 --batch-size 65536 --output resultados_48chars.txt
```
- Threshold alto = muito raro
- 50 resultados
- Batch size máximo para GPU
- Salva em arquivo customizado

### Exemplo 4: Busca Conservadora (CPU ou GPU Fraca)
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5 --batch-size 4096
```
- Batch size menor = menos uso de memória
- Ideal se tiver pouca RAM ou GPU fraca

### Exemplo 5: Com Log Detalhado
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 10 --logfile meu_log.txt --output minhas_seeds.txt
```

## ⚙️ Configuração de GPU

### Primeira Vez (Configurar GPU)
```powershell
.\target\release\gpuseed-rust.exe --reset-config
```
O programa vai perguntar sobre GPU. Escolha:
1. Sim, usar GPU
2. NVIDIA (se tiver placa NVIDIA)

### Próximas Vezes
O programa lembra da configuração. Só execute normalmente:
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5
```

### Mudar Configuração de GPU
```powershell
.\target\release\gpuseed-rust.exe --reset-config
```

## 📊 Monitoramento Durante Execução

O programa mostra em tempo real:
```
Processed: 1,234,567 (150,000/s) | CPU: 45.2% | GPU: 78.5%
```

- **Processed**: Total de iterações processadas
- **X/s**: Velocidade (iterações por segundo)
- **CPU**: Uso de CPU (%)
- **GPU**: Uso de GPU (%)
- **THROTTLE**: Aparece se o sistema de segurança reduzir velocidade

### Parar o Programa
Pressione **Ctrl+C** para parar graciosamente. Os resultados já encontrados serão salvos.

## 📁 Arquivos Gerados

- `seeds_output.txt`: Lista simples de seeds encontrados
  ```
  word1 word2 ... word12 46
  word1 word2 ... word12 47
  ```

- `mnemonics_log.txt`: Log detalhado com timestamps
  ```
  Mnemonic: word1 word2 ... word12
  Total characters: 46
  Time elapsed: 2.5s
  ----------------------------------
  ```

## 🎮 Otimizações por Hardware

### GPU Potente (RTX 3080 Ti, 3090, etc.)
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 10 --batch-size 65536
```

### GPU Média (GTX 1660, RTX 3060, etc.)
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 10 --batch-size 32768
```

### Apenas CPU (Sem GPU ou GPU Desabilitada)
```powershell
.\target\release\gpuseed-rust.exe --threshold 46 --count 5 --batch-size 16384
```

## 🔧 Troubleshooting

### Programa não encontra cargo
```powershell
.\setup_env.ps1
```

### GPU não inicializa
- Verifique se a GPU está funcionando: `nvidia-smi`
- Tente resetar configuração: `--reset-config`
- Verifique se compilou com `--features gpu`

### Performance baixa
- Aumente `--batch-size` (ex: 32768 ou 65536)
- Verifique se GPU está sendo usada (deve mostrar GPU > 0%)
- Verifique se não há throttling (sistema de segurança limitando)

### Quer usar apenas CPU
Quando perguntar sobre GPU, escolha **2** (CPU only)

## 💡 Dicas

1. **Threshold**: Valores entre 44-48 são mais práticos. Acima de 50 pode demorar muito.

2. **Batch Size**: 
   - GPU: 16384-65536
   - CPU: 4096-16384
   - Se der erro de memória, reduza

3. **Count**: Comece com 5-10 para testar. Aumente conforme necessário.

4. **Monitoramento**: Deixe rodando e observe a velocidade. Se estiver muito lento, ajuste os parâmetros.

5. **Múltiplas Execuções**: O programa salva os resultados encontrados. Você pode rodar várias vezes e os resultados se acumulam nos arquivos.

## 🚀 Comando Recomendado para Começar

```powershell
cd rust
.\setup_env.ps1
.\target\release\gpuseed-rust.exe --threshold 46 --count 10 --batch-size 32768
```

Quando perguntar sobre GPU, escolha **1** (usar GPU) e depois **1** (NVIDIA).

Deixe rodar e observe o progresso! 🎯



