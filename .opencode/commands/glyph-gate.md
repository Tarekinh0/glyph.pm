---
description: Gate final avant merge local.
agent: glyph-orchestrator
---

Lance le gate final Glyph.

Contexte:
@AGENTS.md
@Documentation/Architecture-Decision-Records.md
@Documentation/Zero_Data_Architecture.md

État:
!`git status --short`
!`git diff --stat`
!`git diff`

Commandes de vérification à demander aux agents selon le code présent:
- tests unitaires
- lint
- clippy ou équivalent
- tests privacy
- tests sécurité
- vérification absence de secrets
- vérification absence de données réelles dans fixtures
- vérification payload IA labels-only
- vérification non-persistance si applicable

Appelle:
1. glyph-ciso
2. glyph-dpo
3. glyph-qa
4. glyph-release si workflows ou artefacts release sont concernés

Conclusions:
- PASS si tout est vert
- BLOCKED sinon
