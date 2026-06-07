---
description: Revue DPO/RGPD du diff courant.
agent: glyph-dpo
subtask: true
---

Analyse le diff courant au regard du RGPD, de la minimisation et des ADR Glyph.

Fichiers de référence:
@docs/decisions/ADR-001.md
@docs/decisions/ADR-007.md
@docs/decisions/ADR-016.md
@docs/decisions/ADR-019.md
@docs/decisions/ADR-021.md
@AGENTS.md

Diff:
!`git diff --stat`
!`git diff`

Vérifie particulièrement:
- logs
- identifiants persistants
- cookies
- payloads IA
- données de test
- messages d'erreur
- durées de conservation
- AIPD/PIA à mettre à jour

Verdict obligatoire: PASS / BLOCKED uniquement.
