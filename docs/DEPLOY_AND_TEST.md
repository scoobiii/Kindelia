# Guia de Deploy e Testes HVM - Kindelia

Este documento descreve como **rodar, testar e monitorar** os smart contracts de energia e multi-ativos da Kindelia, respeitando a arquitetura atual do projeto.  
Todos os exemplos foram implementados em KDL para execução **paralelizada nativamente** na HVM.

---

## 📦 Estrutura Relevante

Arquivos e pastas relacionados a deploy e testes:

```

      kdl/
      ├── energia/
      │   ├── token_energia.kdl
      │   ├── alienacao_energia.kdl
      │   ├── producao_excedente.kdl
      │   └── liquidacao_energia.kdl
      ├── multiativos/
      │   ├── token_multiativo.kdl
      │   ├── alienacao_multiativo.kdl
      │   └── liquidacao_multiativo.kdl
      src/test/
      └── integracao/
      ├── energia_test.rs
      └── multiativos_test.rs
      scripts/
      ├── run_examples.sh
      ├── test.sh
      └── bench.sh

````

---

## ⚙️ Pré-requisitos

- Rust 1.78+  
- Cargo  
- Docker / Docker Compose (para nós HVM completos)  
- Node.js 20+ (para front-end PWA e scripts JS)  
- `kdl` CLI/HVM executável (`cargo install kdl-cli` ou binário local)  

---

## 🚀 Rodando o Nó Kindelia

### 1. Build Docker

```bash
docker build -t kindelia-node .
````

### 2. Start com Docker Compose

```bash
docker-compose up -d
```

O nó será iniciado, integrando:

* Core HVM
* Rede P2P
* API RPC/WebSocket
* DAO e Treasury

---

## 🏗️ Executando Smart Contracts HVM

### 1. Tokenização de Energia

```bash
# Deploy do token de energia
kdl run kdl/energia/token_energia.kdl --node http://localhost:8080

# Alienação para financiadores
kdl run kdl/energia/alienacao_energia.kdl --node http://localhost:8080

# Registro de produção excedente
kdl run kdl/energia/producao_excedente.kdl --node http://localhost:8080

# Liquidação para financiadores
kdl run kdl/energia/liquidacao_energia.kdl --node http://localhost:8080
```

### 2. Tokenização Multi-ativos

```bash
# Deploy do token multi-ativo
kdl run kdl/multiativos/token_multiativo.kdl --node http://localhost:8080

# Alienação de ativos
kdl run kdl/multiativos/alienacao_multiativo.kdl --node http://localhost:8080

# Liquidação de contratos
kdl run kdl/multiativos/liquidacao_multiativo.kdl --node http://localhost:8080
```

> **Nota:** Todos os contratos rodam **paralelamente**, utilizando a execução nativa da HVM.

---

## 🧪 Testes Automatizados

### 1. Testes de integração HVM

```bash
# Rodar todos os testes de integração
cargo test --test integracao/energia_test.rs
cargo test --test integracao/multiativos_test.rs
```

### 2. Testes de benchmark

```bash
# Benchmarks de execução HVM
./scripts/bench.sh
./scripts/bench_cmp.py
```

### 3. Cobertura de testes

```bash
./scripts/cov.sh
```

---

## 🔗 Observabilidade

* **Monitoramento de nós HVM:** `src/analytics/monitor.rs`
* **Relatórios de produção:** `kdl/energia/producao_excedente.kdl` gera logs mensais
* **Alertas e auditoria:** Integrados via DAO e contrato de treasury

---

## 📚 Dicas de Operação

* Use **nodes diferentes** para paralelizar testes sem afetar produção.
* Cada execução KDL registra **hash reversível**, permitindo auditoria completa.
* Para produção, recomenda-se **Docker Swarm ou Kubernetes** para alta disponibilidade.

---

## 🔧 Referências

* [README.md](../README.md)
* [WHITEPAPER.md](../WHITEPAPER.md)
* [ARCHITECTURE.md](ARCHITECTURE.md)
* Exemplos KDL em `kdl/energia` e `kdl/multiativos`

---

## ✅ Conclusão

Seguindo este guia você será capaz de:

1. Rodar a HVM paralelizada da Kindelia
2. Executar smart contracts de energia e multi-ativos
3. Testar, auditar e monitorar a execução de forma segura
4. Integrar com DAO, treasury e front-end PWA

> Kindelia transforma **recursos reais em ativos digitais**, mantendo a **segurança, paralelização e auditabilidade** nativa da HVM.



