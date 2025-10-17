# Kindelia DAO Structure

## Estrutura de Governança
- **Constitution:** `/dao/constitution.md`
- **Roles:** `/dao/roles.yaml`
- **Proposals:** `/dao/proposals/`
- **Treasury Tokenomics:** `/dao/dao_tokenomics.toml`

### Papéis Principais
| Papel | Permissões |
|-------|-------------|
| Founder | Seed Allocation + Code Merge |
| Governor | Criação e votação de propostas |
| Auditor | Validação de segurança e compliance |
| Developer | Envio de PRs e melhorias técnicas |
| Community | Voto em parâmetros econômicos |

### Fluxo de Proposta (Mermaid)
```mermaid
graph TD
    A[Proposta criada] --> B[Validação DAO]
    B --> C[Votação on-chain]
    C --> D{Aprovada?}
    D -->|Sim| E[Execução automática]
    D -->|Não| F[Arquivada]
