#!/bin/bash
BASE="./docs/sprints"
SPRINTS=(
  "Sprint_1_Reestruturação"
  "Sprint_2_HVM_Runtime"
  "Sprint_3_API_Persistencia"
  "Sprint_4_DAO_Governanca"
  "Sprint_5_Tokenomics"
  "Sprint_6_UX_CLI_60+"
  "Sprint_7_CI_CD_Testes"
  "Sprint_8_Deploy_Seed"
)

for S in "${SPRINTS[@]}"; do
  mkdir -p "$BASE/$S"
  for F in GOALS.md TASKS.md TEST_PLAN.md RESULTS.md; do
cat <<EOF > "$BASE/$S/$F"
# $(basename "$F" .md) – ${S//_/ }

**Sprint:** $S  
**Gerado:** $(date '+%Y-%m-%d')  
**Responsável:** @core-dev / @qa-team / @dao-board  

---

## Descrição
(Preencher com detalhes específicos da sprint)

## Notas Técnicas
(Referências de commits, PRs, issues)

## Próximos Passos
(Listar dependências da próxima sprint)
EOF
  done
done

echo "✅ Estrutura de Sprints criada em $BASE"
