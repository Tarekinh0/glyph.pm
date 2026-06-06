---
description: Lance un cycle complet de story Glyph avec DPO, CISO, DevSecOps, QA et Release.
agent: glyph-orchestrator
---

Objectif de sprint ou story demandée:
$ARGUMENTS

Contexte obligatoire à lire:
@README.md
@Documentation/Architecture-Decision-Records.md
@Documentation/Zero_Data_Architecture.md
@AGENTS.md

État courant:
!`git status --short`
!`git log --oneline -5`

Tu dois piloter le cycle suivant:

1. Choisir ou reformuler une story verticale minimale.
2. Créer un dossier docs/implementation/sprints/GLYPH-XXXX/.
3. Rédiger story.md avec:
   - objectif
   - périmètre
   - hors périmètre
   - ADR impactés
   - critères d'acceptation
   - tests attendus

4. Appeler glyph-dpo.
5. Appeler glyph-ciso.
6. Si DPO ou CISO bloque, arrêter et produire closure.md BLOCKED.
7. Si les deux passent, appeler glyph-devsecops pour implémentation.
8. Appeler glyph-ciso pour validation du diff.
9. Si CISO échoue, renvoyer au DevSecOps, maximum 2 boucles.
10. Appeler glyph-dpo pour validation du diff.
11. Si DPO échoue, renvoyer au DevSecOps, maximum 2 boucles.
12. Appeler glyph-qa.
13. Appeler glyph-release si CI/CD, dépendances, build, SBOM ou release sont touchés.
14. Sauvegarder les rapports complets des agents dans des fichiers (ciso-review.md, dpo-review.md, qa-review.md, release-review.md) dans le dossier du sprint.
15. Produire closure.md avec PASS ou FAIL.

Ne modifie jamais les ADR pour faire passer une story.
