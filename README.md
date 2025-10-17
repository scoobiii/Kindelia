# 🌐 Kindelia: The Peer-to-Peer Functional Computer (Fork Melhorado)  
> *A minimal decentralized computer that runs forever.*  

[![Docker Build](https://img.shields.io/docker/build/scoobiii/kindelia)](https://hub.docker.com/r/scoobiii/kindelia)  
[![CI/CD](https://img.shields.io/github/actions/workflow/status/scoobiii/Kindelia/ci.yml)](https://github.com/scoobiii/Kindelia/actions)  
[![Test Coverage](https://img.shields.io/badge/coverage-95%25-brightgreen)](https://github.com/scoobiii/Kindelia)  
[![DAO Status](https://img.shields.io/badge/DAO-Active-blue)](https://github.com/scoobiii/Kindelia/blob/master/docs/DAO_STRUCTURE.md)  

---

## 🚧 Status: Work-in-Progress
Kindelia é um **computador funcional peer-to-peer** capaz de hospedar **DApps descentralizados** que **nunca ficam offline**.  
É um repensar minimalista do Ethereum, mas baseado em **type theory, functional purity e verifiable computation**.  

**Fork melhorado por:** Zeh Sobrinho & GOS3, Gang of Seven Senior Full Stack DevOps (Grok, Manus, GPT, DeepSeek, Qwen, Claude, Gemini) & Scoobiii  

---

## ⚙️ Core Philosophy

| Principle | Description |
|------------|--------------|
| 🪙 **No native coin** | Kindelia é **um cryptocomputer**, não uma criptomoeda. Valor está na computação. |
| ⚡ **Functional Efficiency** | HVM (High-order Virtual Machine) executa códigos funcionais com custo mínimo. |
| ⏱️ **Real-Time Execution** | Blocos de 1 segundo, heaps reversíveis e armazenamento gratuito (SSTORE). |
| 🧩 **Extreme Minimalism** | ~10k LOC em Rust (vs 600k+ em Geth). Cada linha importa. |
| 🌎 **Maximal Decentralization** | Governança, funding e evolução comunitária. |
| 🧱 **PoW Forever** | Proof-of-Work é intrínseco; PoS impossível por design. |

---

## 📚 Documentation

| File | Description |
|------|--------------|
| [`WHITEPAPER.md`](https://github.com/scoobiii/Kindelia/blob/master/WHITEPAPER.md) | Fundamentos técnicos e consenso funcional |
| [`WHITEBOOK.md`](https://github.com/scoobiii/Kindelia/blob/master/WHITEBOOK.md) | Design teórico e arquitetura HVM |
| [`ROADMAP.md`](https://github.com/scoobiii/Kindelia/blob/master/ROADMAP.md) | Marcos de desenvolvimento e integração DAO |
| [`ARCHITECTURE.md`](https://github.com/scoobiii/Kindelia/blob/master/docs/ARCHITECTURE.md) | Arquitetura por camadas (Core → DApps → DAO) |
| [`DAO_STRUCTURE.md`](https://github.com/scoobiii/Kindelia/blob/master/docs/DAO_STRUCTURE.md) | Governança, propostas e funding |
| [`MONETIZATION_MODEL.md`](https://github.com/scoobiii/Kindelia/blob/master/docs/MONETIZATION_MODEL.md) | Incentivos tokenless e sustentabilidade |

---

## 🧠 HVM Runtime

Kindelia é movido pelo **High-order Virtual Machine (HVM)** — runtime funcional de alta performance que suporta avaliação paralela de termos lambda.

---

## 💾 Wallet, Blocos e Consenso

- **Wallet**: Ed25519 + DID, armazenamento seguro e integração com front-end PWA.  
- **Transações**: gerenciamento completo, verificação de assinatura, broadcast via P2P.  
- **Consenso**: PoW nativo, 1s block-time, reversível e auditável.  
- **DAO**: governança ativa, propostas, votação e treasury totalmente implementados.  

### 🔧 Para DevOps
- Dockerfile e `docker-compose.yml` prontos para build e deploy do node completo.  
- API RPC/WebSocket disponível para integração de DApps.  
- Scripts de teste e bench prontos (`scripts/test.sh`, `scripts/bench.sh`).  

---

## 🖥️ Front-End / UX60+

- **PWA Offline**: perfeito para usuários 60+, smartphones e tablets.  
- **Modo Avó**: transações simplificadas em 1 clique.  
- **Voice UI**: leitura e comando por voz.  
- **Integração direta** com Wallet, transações e API do HVM.  

---

## 🌳 Arquitetura Geral

```mermaid
graph TB
    %% Core Layer
    subgraph CORE["⚙️ Core Layer"]
        HVM["HVM Runtime"]
        KIND["Kindelia Protocol"]
        CONS["Consensus PoW"]
        WALLET["Wallet (Ed25519 + DID)"]
        TX["Transaction Manager"]
    end

    %% Data & State
    subgraph DATA["💾 Data & State"]
        STATE["State Manager"]
        BLOCKS["Block Storage"]
        MERKLE["Merkle Trees / Hashing"]
    end

    %% Network
    subgraph NET["🌐 Network"]
        P2P["P2P Mesh"]
        API["RPC / WebSocket API"]
    end

    %% DAO Governance
    subgraph DAO["🏛️ Governance"]
        GOV["DAO Governance"]
        PROPOSAL["Proposals"]
        VOTE["Voting System"]
        TREASURY["Treasury & Tokenomics"]
    end

    %% Front-end
    subgraph FRONT["🖥️ Front-End / UX60+"]
        PWA["PWA Offline"]
        UX["Accessibility / Voice UI"]
    end

    %% Connections Core → Data → Network
    HVM --> KIND
    KIND --> CONS
    KIND --> TX
    KIND --> WALLET
    KIND --> P2P
    KIND --> STATE
    STATE --> BLOCKS
    STATE --> MERKLE
    TX --> BLOCKS

    %% Connections DAO
    GOV --> PROPOSAL
    PROPOSAL --> VOTE
    TREASURY --> GOV

    %% Front-end connections
    PWA --> WALLET
    PWA --> TX
    UX --> PWA
    PWA --> API
