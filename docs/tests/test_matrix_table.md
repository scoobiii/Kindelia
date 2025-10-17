# Test Matrix & Execution Plan

## Matrix por Módulo x Tipo de Teste

| Módulo | Unit | Integration | Property | Performance | UX/A11y |
|--------|:----:|:-----------:|:--------:|:-----------:|:-------:|
| HVM Core Runtime | ✅ | ✅ | ✅ | ✅ | - |
| Kindelia VM Layer | ✅ | ✅ | ✅ | ✅ | - |
| Consensus Engine | ✅ | ✅ | ✅ | ✅ | - |
| Storage (KDB) | ✅ | ✅ | - | ✅ | - |
| Wallet SDK + CLI | ✅ | ✅ | - | ✅ | ✅ |
| Web3 Gateway API | ✅ | ✅ | - | ✅ | - |
| DAO Governance Contracts | ✅ | ✅ | ✅ | - | - |
| Quantum AI Module | - | ✅ (sim) | ✅ (sim) | - | - |
| IDE Plugin | ✅ | ✅ | - | - | ✅ |
| UX/UI Global | - | ✅ | - | ✅ | ✅ |

## Execução (CI)
- `cargo test` → Unit + some integration  
- `scripts/test.sh` → Full integration suite (Docker Compose)  
- `scripts/metrics_report.py` → Collect latencies and coverage reports  
- Codecov upload: on success → badge update

## Acceptance Criteria
- All `Blocker` tests must be green to allow merge to `main`.  
- Coverage decrease >1% triggers rollback and hotfix branch.
