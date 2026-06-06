# GLYPH-000: Mettre en place la gouvernance OpenCode multi-agents

## Objectif
Créer les agents glyph-orchestrator, glyph-dpo, glyph-ciso, glyph-devsecops, glyph-qa, glyph-release, les commandes OpenCode et AGENTS.md pour mettre en place la gouvernance multi-agents.

## Périmètre
- Création des agents dans `.opencode/agents/` (orchestrator, dpo, ciso, devsecops, qa, release).
- Création des commandes `.opencode/commands/glyph-sprint.md` et `.opencode/commands/glyph-gate.md`.
- Mise à jour ou création de `AGENTS.md`.
- Création des templates dans `docs/implementation/templates/`.
- Configuration des permissions pour restreindre les modifications de code par les reviewers et les modifications d'ADR par le DevSecOps.

## Hors périmètre
- Git push automatique.
- Déploiement automatique.

## ADR impactés
- **ADR-001** : Architecture stateless zero-knowledge.
- **ADR-020** : Stratégie CI/CD — Forgejo, GitHub Actions, SAST/DAST.
- **ADR-031** : Choix du toolchain IaC — Ansible + Terraform vs NixOS.
- **ADR-032** : Provenance du binaire — SLSA, Sigstore, Rekor, SBOM.

## Critères d'acceptation
- Les reviewers (DPO, CISO, QA, Release) ne peuvent pas modifier le code.
- Le DevSecOps ne peut pas modifier les ADR.
- Les commandes incluent le contexte obligatoire : README, ADR, Zero_Data_Architecture.

## Tests attendus
- Vérification des permissions des agents dans `.opencode/agents/`.
- Vérification du contenu des commandes `.opencode/commands/`.
- Vérification de la présence et du contenu de `AGENTS.md` et des templates.
