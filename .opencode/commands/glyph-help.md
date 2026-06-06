---
description: Liste les commandes disponibles et explique brièvement le workflow.
agent: glyph-orchestrator
---

Affiche l'aide sur les commandes Glyph et le workflow.

Instructions:
Affiche un résumé clair des commandes disponibles pour gérer le projet Glyph :

- `/glyph-next` : Trouve le prochain item READY dans le backlog et lance le sprint.
- `/glyph-sprint <ID>` : Lance un cycle complet de sprint pour l'item spécifié.
- `/glyph-backlog-status` : Affiche l'état macro du projet (bloqués, prêts, etc.).
- `/glyph-backlog-refine <instructions>` : Affine le backlog et la roadmap.
- `/glyph-gate` : Gate final de validation avant merge.
- `/glyph-help` : Affiche cette aide.

Explique brièvement le workflow séquentiel :
1. Affinage du backlog (`/glyph-backlog-refine`).
2. Vérification de l'état (`/glyph-backlog-status`).
3. Lancement du prochain sprint (`/glyph-next` ou `/glyph-sprint GLYPH-XXX`).
4. Déroulement du sprint (Orchestrator -> DPO/CISO -> DevSecOps -> CISO/DPO -> QA/Release).
5. Validation finale (`/glyph-gate`).
