---
description: Permet à l'Orchestrateur d'affiner le backlog (YAML, roadmap, risques) sans toucher au code applicatif.
agent: glyph-orchestrator
---

Affine le backlog et la roadmap.

Arguments:
$ARGUMENTS

Contexte:
@docs/implementation/backlog/glyph-v1-backlog.yaml
@docs/implementation/backlog/glyph-v1-roadmap.md
@AGENTS.md

Instructions:
1. Prends en compte les arguments fournis pour affiner le backlog.
2. Modifie `docs/implementation/backlog/glyph-v1-backlog.yaml` et/ou `docs/implementation/backlog/glyph-v1-roadmap.md` en conséquence.
3. Mets à jour les statuts, les dépendances, les risques, ou ajoute de nouveaux items.
4. Ne touche à aucun fichier de code applicatif.
5. Assure-toi que le YAML reste valide.
