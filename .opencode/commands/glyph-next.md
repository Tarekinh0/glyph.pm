---
description: Lit le backlog canonique, trouve le prochain item READY, crée le dossier de sprint, et lance l'Orchestrateur.
agent: glyph-orchestrator
---

Trouve le prochain item READY dans le backlog et lance le sprint.

Contexte:
@docs/implementation/backlog/glyph-v1-backlog.yaml
@docs/implementation/backlog/glyph-v1-roadmap.md
@AGENTS.md

Instructions:
1. Lis le backlog canonique (`docs/implementation/backlog/glyph-v1-backlog.yaml`).
2. Trouve le prochain item avec le statut `READY` (dépendances OK, inputs fournis).
3. Si aucun item n'est `READY`, affiche un message expliquant pourquoi (ex: dépendances bloquantes, inputs manquants) et arrête-toi.
4. Si un item est `READY`, extrais son ID (ex: GLYPH-001).
5. Crée le dossier de sprint `docs/implementation/sprints/<ID>/`.
6. Lance la commande `/glyph-sprint <ID>` pour démarrer le cycle de sprint sur cet item.
