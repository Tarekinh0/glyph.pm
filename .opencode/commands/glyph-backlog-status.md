---
description: Affiche l'état macro du projet (items bloqués, inputs manquants, prochain item).
agent: glyph-orchestrator
---

Affiche l'état macro du backlog Glyph.

Contexte:
@docs/implementation/backlog/glyph-v1-backlog.yaml
@docs/implementation/backlog/glyph-v1-roadmap.md

Instructions:
1. Analyse le fichier `docs/implementation/backlog/glyph-v1-backlog.yaml`.
2. Génère un rapport d'état macro contenant :
   - Le prochain item `READY` à traiter.
   - La liste des items `BLOCKED` avec la raison (dépendances, inputs manquants).
   - La liste des items `TODO` en attente.
   - La liste des items `IN_PROGRESS` ou `DONE`.
3. Résume l'avancement par rapport à la `roadmap.md`.
