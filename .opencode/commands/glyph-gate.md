---
description: Gate final avant merge local.
agent: glyph-orchestrator
---

Lance le gate final Glyph.

Contexte:
@AGENTS.md
@docs/decisions/README.md
@docs/Zero_Data_Architecture.md

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

Appelle les agents pour effectuer leurs reviews et générer les fichiers correspondants dans le dossier du sprint courant (ou à la racine si non applicable) :
1. glyph-ciso -> `ciso-review.md`
2. glyph-dpo -> `dpo-review.md`
3. glyph-qa -> `qa-review.md`
4. glyph-release -> `release-review.md` (si workflows ou artefacts release sont concernés)

Conclusions:
- L'Orchestrateur lit tous les fichiers de review.
- PASS si tout est vert.
- BLOCKED sinon, avec la liste des points bloquants.
