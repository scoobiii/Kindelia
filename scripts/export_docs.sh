#!/bin/bash
# Responsável: DevOps Team
# Descrição: Converte documentação Kindelia em HTML e PDF
# Requerimentos: pandoc + mermaid-cli

mkdir -p build/docs
for f in $(find docs -name "*.md"); do
  name=$(basename $f .md)
  echo "📄 Exportando $name..."
  pandoc "$f" -s -o "build/docs/$name.html"
done
