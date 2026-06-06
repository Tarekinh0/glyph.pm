---
description: Lance un cycle complet de story Glyph avec DPO, CISO, DevSecOps, QA et Release.
agent: glyph-orchestrator
---

ID de l'item du backlog à traiter (ex: GLYPH-001) ou description de la story:
$ARGUMENTS

Contexte obligatoire à lire:
@docs/implementation/backlog/glyph-v1-backlog.yaml
@README.md
@docs/decisions/README.md
@Documentation/Zero_Data_Architecture.md
@AGENTS.md

État courant:
!`git status --short`
!`git log --oneline -5`

Tu dois piloter le cycle suivant:

1. Si un ID de backlog est fourni (ex: GLYPH-XXX), extraire ses informations depuis `docs/implementation/backlog/glyph-v1-backlog.yaml`. Sinon, utiliser la description fournie.
2. Créer un dossier `docs/implementation/sprints/<ID>/` (ou un nom généré si pas d'ID).
3. Rédiger `story.md` dans ce dossier avec:
   - objectif
   - périmètre
   - hors périmètre
   - ADR impactés
   - critères d'acceptation
   - tests attendus

4. Appeler glyph-dpo pour rédiger `dpo-requirements.md`.
5. Appeler glyph-ciso pour rédiger `ciso-requirements.md`.
6. Si DPO ou CISO bloque, arrêter et produire `closure.md` BLOCKED.
7. Si les deux passent, appeler glyph-devsecops pour implémentation et rédaction de `dev-notes.md`.
8. Appeler glyph-ciso pour validation du diff et rédaction de `ciso-review.md`.
9. Si CISO échoue, renvoyer au DevSecOps, maximum 2 boucles.
10. Appeler glyph-dpo pour validation du diff et rédaction de `dpo-review.md`.
11. Si DPO échoue, renvoyer au DevSecOps, maximum 2 boucles.
12. Appeler glyph-qa pour validation et rédaction de `qa-review.md`.
13. Appeler glyph-release si CI/CD, dépendances, build, SBOM ou release sont touchés, pour rédaction de `release-review.md`.
14. Produire `closure.md` avec PASS ou FAIL.

Ne modifie jamais les ADR pour faire passer une story.
