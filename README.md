
# 🌐 Kindelia
> *A peer-to-peer functional blockchain & cryptocomputer, improved by Zeh Sobrinho & GOS3, Gang of Seven Senior Full Stack DevOps (Grok, Manus, GPT, DeepSeek, Qwen, Claude, Gemini) and Scoobiii.*

---

## 🚧 Status: Work-in-Progress / Improved Fork

Kindelia agora é uma **blockchain completa**, com:

* Wallet segura (Ed25519 + DID)
* Criação, assinatura e envio de transações
* Blocos imutáveis e rastreáveis
* Consenso Proof-of-Work distribuído
* Front-end PWA offline com UX60+ (“Modo Avó”, Voice UI)
* DAO de governança funcional (**100% implementada**)
* Docker + Docker Compose para deployment rápido e isolado

> A DAO implementa governança total: propostas, votação, execução de decisões e integração com treasury e tokenomics.

---

## ⚙️ Core Philosophy

| Principle                              | Description                                                              |
| -------------------------------------- | ------------------------------------------------------------------------ |
| 🪙 **Tokenless Functional Blockchain** | Valor está na computação distribuída, não em moeda nativa.               |
| ⚡ **Functional Efficiency**            | High-order Virtual Machine (HVM) com execução determinística e paralela. |
| ⏱️ **Real-Time Execution**             | 1s blocks, heaps reversíveis, SSTORE zero-cost.                          |
| 🧩 **Extreme Minimalism**              | ~10k LOC em Rust; código enxuto e auditável.                             |
| 🌎 **Maximal Decentralization**        | Rede, governança e evolução controladas por DAO.                         |
| 🧠 **Consensus PoW**                   | Proof-of-Work puro, resistente a ataques.                                |

---

## 🔹 Features deste Fork

| Feature                    | Status                                                            |
| -------------------------- | ----------------------------------------------------------------- |
| Wallet (Ed25519 + DID)     | ✅ Completa, PWA offline e integração API                          |
| Transações & Execução HVM  | ✅ Determinísticas, assinadas, rastreáveis                         |
| Blocos & Blockchain        | ✅ Estrutura completa, consulta via API                            |
| Consenso                   | ✅ PoW distribuído, nodes sincronizados                            |
| UX60+ / Accessibility      | ✅ Fontes grandes, Modo Avó, TTS, layout simplificado              |
| DAO Governance             | ✅ **100% implementada**: propostas, votação, treasury, tokenomics |
| Front-end PWA              | ✅ Offline, mobile-friendly, integração wallet                     |
| Docker / DevOps Deployment | ✅ Containerized backend + front-end via Docker Compose            |

---

## 📚 Documentação (links diretos)

| File                                                                                                 | Description                                            |
| ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| [WHITEPAPER.md](https://github.com/scoobiii/Kindelia/blob/master/WHITEPAPER.md)                      | Fundamentos técnicos e consenso funcional              |
| [WHITEBOOK.md](https://github.com/scoobiii/Kindelia/blob/master/WHITEBOOK.md)                        | Design teórico e arquitetura HVM                       |
| [ROADMAP.md](https://github.com/scoobiii/Kindelia/blob/master/ROADMAP.md)                            | Milestones do fork e integração DAO                    |
| [ARCHITECTURE.md](https://github.com/scoobiii/Kindelia/blob/master/docs/ARCHITECTURE.md)             | Arquitetura Core → DApps → DAO                         |
| [DAO_STRUCTURE.md](https://github.com/scoobiii/Kindelia/blob/master/docs/DAO_STRUCTURE.md)           | Governança, propostas, treasury e tokenomics           |
| [MONETIZATION_MODEL.md](https://github.com/scoobiii/Kindelia/blob/master/docs/MONETIZATION_MODEL.md) | Incentivos tokenless e sustentabilidade do ecossistema |
| [front_end.md](https://github.com/scoobiii/Kindelia/blob/master/docs/front_end.md)                   | Guia PWA, UX60+, integração wallet, offline mode       |

---

## 🧠 HVM Runtime

O **High-order Virtual Machine (HVM)** suporta:

* Execução funcional paralela
* Contratos Lambda determinísticos e auditáveis
* Integração total com API REST e front-end PWA
* Logs e métricas para DevOps e monitoramento de rede

---

## 🐳 Docker & Deployment

O projeto inclui **Dockerfile** e **docker-compose.yml**, permitindo:

* Rodar backend Rust isolado
* Servir front-end PWA no mesmo container ou separado
* Facilitar DevOps, CI/CD e testes de integração

### Rodando com Docker

1. Build da imagem:

```bash
docker build -t kindelia:latest .
```

2. Rodar container único:

```bash
docker run -p 8080:8080 kindelia:latest
```

3. Com Docker Compose (backend + front-end):

```bash
docker-compose up --build
```

> O Compose já define volumes para persistência de blockchain, rede PoW, e mapeia portas para API e PWA.

---

## 🚀 Como Rodar Localmente

**Sem Docker:**

```bash
# Backend Rust
cargo build --release
cargo run --release

# Front-end PWA
cd src
npm install
npm run dev
```

**Com Docker:** siga o passo anterior.

---

## 💡 Para Usuário / DevOps

* **Usuário:** criar wallet, enviar e assinar transações, consultar blocos via PWA ou CLI.
* **DevOps:** monitorar nodes, auditar blocos, atualizar protocolo via DAO, CI/CD via Docker.
* **Drex / CBDC:** suporte inicial para identidade digital, transações rápidas e auditáveis.
